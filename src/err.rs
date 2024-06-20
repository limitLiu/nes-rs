pub type Result<T> = std::result::Result<T, EmuError>;

#[derive(Debug)]
pub enum EmuError {
  IOError(String),
}

impl std::error::Error for EmuError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self)
  }
}

impl std::fmt::Display for EmuError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      EmuError::IOError(ref cause) => write!(f, "{}", cause),
    }
  }
}
