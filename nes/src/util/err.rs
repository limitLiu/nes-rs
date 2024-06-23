pub type Result<T> = std::result::Result<T, EmuError>;

#[derive(Debug)]
pub enum EmuError {
  Other(String),
  IO(std::io::Error),
}

impl std::error::Error for EmuError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self)
  }
}

impl std::fmt::Display for EmuError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      EmuError::IO(ref cause) => write!(f, "{}", cause),
      EmuError::Other(ref cause) => write!(f, "{}", cause),
    }
  }
}

impl From<String> for EmuError {
  fn from(value: String) -> EmuError {
    EmuError::Other(value)
  }
}

impl From<std::io::Error> for EmuError {
  fn from(value: std::io::Error) -> EmuError {
    EmuError::IO(value)
  }
}
