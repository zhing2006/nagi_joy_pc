mod config;
mod joy_data;
mod message;
mod service;

use tokio::net::UdpSocket;
use tokio::runtime::Builder;

use service::Service;

fn main() -> anyhow::Result<()> {
  // Initialize the logger.
  init_log()?;

  // Load configure from `conf/config.toml`.
  let config = config::Config::with_file("conf/config.toml")?;
  #[cfg(debug_assertions)]
  log::debug!("Configurations: {:#?}", config);

  // Start tokio runtime.
  let rt = Builder::new_current_thread()
    .enable_io()
    .build()?;

  // Run the service.
  rt.block_on(run_service(config))?;

  Ok(())
}

/// Initialize the logger.
/// return: The result of the logger initialization.
fn init_log() -> anyhow::Result<()> {
  // Create the directory if it does not exist.
  std::fs::create_dir_all("./logs")?;

  // Rename the old log file.
  if std::fs::metadata("./logs/nagi_joy_pc.log").is_ok() {
    std::fs::rename("./logs/nagi_joy_pc.log", "./logs/ngai_joy_pc.prev.log")?;
  }

  // Initialize the logger.
  let log_config = simplelog::ConfigBuilder::new()
    .set_time_format_custom(simplelog::format_description!("[hour]:[minute]:[second].[subsecond]"))
    .build();

  simplelog::CombinedLogger::init(
    vec![
      simplelog::TermLogger::new(
        if cfg!(debug_assertions) {
          simplelog::LevelFilter::Debug
        } else {
          simplelog::LevelFilter::Info
        },
        log_config.clone(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
      ),
      simplelog::WriteLogger::new(
        simplelog::LevelFilter::Info,
        log_config.clone(),
        std::fs::File::create("./logs/nagi_joy_pc.log")?,
      ),
    ]
  )?;

  Ok(())
}

/// The service main loop.
/// param config: The application configuration.
/// return: The result of the service execution.
async fn run_service(config: config::Config) -> anyhow::Result<()> {
  // Create a UDP socket.
  let addr = format!("{}:{}", config.service.host, config.service.port);
  let socket = UdpSocket::bind(addr).await?;
  log::info!("UDP service started on: {}", socket.local_addr()?);

  // Create a new service instance.
  let mut service = Service::new(config)?;

  loop {
    tokio::select! {
      result = socket.recv_from(&mut service.rx_buf) => {
        match result {
          Ok((amt, src_addr)) => {
            log::debug!("Received {} bytes from: {}", amt, src_addr);

            service.parse_request(src_addr, amt);
            if service.tx_len > 0 {
              let data = &service.tx_buf[..service.tx_len];
              if let Err(e) = socket.send_to(data, src_addr).await {
                log::error!("Failed to send data, error: {}", e);
              }
              log::debug!("Sent {} bytes to: {}", service.tx_len, src_addr);
            }
          },
          Err(e) => {
            log::error!("Failed to receive data: {}", e);
          },
        }
      },
      _ = tokio::signal::ctrl_c() => {
        log::info!("Received Ctrl-C signal. Shutting down...");
        break;
      },
    }
  }

  Ok(())
}
