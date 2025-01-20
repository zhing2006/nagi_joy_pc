
/// The application configuration.
#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Config {
  pub service: ServiceConfig,
  pub joystick: JoystickConfig,
}

/// The service configuration.
#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ServiceConfig {
  pub host: String,
  pub port: u16,
}

/// The joystick button configuration.
#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickButtonConfig {
  pub index: u32,
  pub inverted: bool,
}

/// The joystick axis configuration.
#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickAxisConfig {
  pub index: u32,
  pub inverted: bool,
  pub min: i32,
  pub max: i32,
}

/// The joystick configuration.
#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickConfig {
  pub index: u32,
  pub buttons: Vec<JoystickButtonConfig>,
  pub axes: Vec<JoystickAxisConfig>,
}

/// The implementation of the Config struct.
impl Config {

  /// Create a new Config instance from a file.
  /// param file: The path to the configuration file.
  /// return: The Config instance.
  pub fn with_file<T>(file_path: T) -> anyhow::Result<Self>
  where
    T: AsRef<std::path::Path>,
  {
    let file_path = file_path.as_ref();
    let config = std::fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config)?;

    Ok(config)
  }

}