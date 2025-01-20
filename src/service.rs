use std::net::SocketAddr;

use crate::config::Config;
use crate::joy_data::Joystick;
use crate::message::{
  AsBytes,
  FromBytes,
  MessageHeader,
  MessageMajorId,
  CommonMessageMinorId,
  PingRequest,
  PongResponse,
  JoystickMessageMinorId,
  JoystickSync,
  JoystickAck,
};

/// The service struct.
pub struct Service {
  /// The configuration.
  #[allow(dead_code)]
  pub config: Config,
  /// The locked on client address.
  pub locked_on_address: Option<SocketAddr>,
  /// The joystick instance.
  pub joystick: Joystick,
  /// Buffer for received data.
  pub rx_buf: [u8; 1024],
  /// Buffer for sending data.
  pub tx_buf: [u8; 1024],
  /// The sending data length.
  pub tx_len: usize,
}

/// Service implementation.
impl Service {

  /// Create a new service instance.
  /// param config: The configuration.
  /// return: The new service instance.
  pub fn new(config: Config) -> anyhow::Result<Self> {
    let joystick = Joystick::new(config.joystick.index)?;
    let rx_buf = [0u8; 1024];
    let tx_buf = [0u8; 1024];

    Ok(Self {
      config,
      locked_on_address: None,
      joystick,
      rx_buf,
      tx_buf,
      tx_len: 0,
    })
  }

  /// Parse the request.
  /// param src_addr: The source address.
  /// param rx_len: The received data length.
  pub fn parse_request(&mut self, src_addr: SocketAddr, rx_len: usize) {
    if let Some(locked_on_address) = self.locked_on_address {
      if locked_on_address != src_addr {
        log::warn!("Received data from an unknown client: {}. Skip it.", src_addr);
        self.tx_len = 0;
        return;
      }
    }

    let header = unsafe { *(self.rx_buf.as_ptr() as *const MessageHeader) };
    let payload_len = rx_len - std::mem::size_of::<MessageHeader>();
    if header.length as usize != payload_len {
      log::warn!("Invalid message length: {} != {}, ID[{}, {}]", header.length, payload_len, header.major_id, header.minor_id);
      self.tx_len = 0;
      return;
    }

    match header.major_id {
      MessageMajorId::COMMON => {
        match header.minor_id {
          CommonMessageMinorId::PING => {
            let request = <PingRequest as FromBytes<_>>::from_bytes(&self.rx_buf[..rx_len]);
            if let Some(request) = request {
              if request.magic == PingRequest::magic_number() {
                // Try to locked on the client.
                if self.locked_on_address.is_none() {
                  log::info!("Locked on client: {}", src_addr);
                  self.locked_on_address = Some(src_addr);
                }

                // Build the response.
                let response = PongResponse::new();
                let bytes = response.as_bytes();
                self.tx_len = bytes.len();
                self.tx_buf[..bytes.len()].copy_from_slice(bytes);
              } else {
                log::warn!("Invalid ping magic number: 0x{:X}", request.magic);
                self.tx_len = 0;
              }
            } else {
              log::warn!("Failed to parse ping request.");
              self.tx_len = 0;
            }
          }
          _ => {
            log::warn!("Unknown common message minor ID: {}", header.minor_id);
            self.tx_len = 0;
          }
        }
      }
      MessageMajorId::JOYSTICK => {
        if self.locked_on_address.is_none() {
          log::warn!("Received joystick message before locking on a client. Skip it.");
          self.tx_len = 0;
          return;
        }
        match header.minor_id {
          JoystickMessageMinorId::DATA_SYNC => {
            let sync = <JoystickSync as FromBytes<_>>::from_bytes(&self.rx_buf[..rx_len]);
            if let Some(sync) = sync {
              // Update joystick.
              match self.joystick.update(&sync.data, &self.config.joystick) {
                Ok(_) => {
                  log::debug!("Joystick data updated.");
                },
                Err(e) => {
                  log::error!("Failed to update joystick data: {}", e);
                },
              }

              // Build the ack.
              let ack = JoystickAck::new((JoystickAck::O << 8) | JoystickAck::K);
              let bytes = ack.as_bytes();
              self.tx_len = bytes.len();
              self.tx_buf[..bytes.len()].copy_from_slice(bytes);
            } else {
              log::warn!("Failed to parse joystick sync message.");
              self.tx_len = 0;
            }
          }
          _ => {
            log::warn!("Unknown joystick message minor ID: {}", header.minor_id);
            self.tx_len = 0;
          }
        }
      }
      _ => {
        log::warn!("Unknown message major ID: {}", header.major_id);
        self.tx_len = 0;
      }
    }
  }

}