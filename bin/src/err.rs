use super::prelude::*;
use nes::util::err::EmuError;
use sdl2::{video, IntegerOrSdlError};

pub type Result<T> = std::result::Result<T, SDLError>;

#[derive(Debug)]
pub enum SDLError {
  Window(video::WindowBuildError),
  IntegerOrSdl(IntegerOrSdlError),
  Emulator(EmuError),
  Other(String),
}

impl std::error::Error for SDLError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self)
  }
}

impl std::fmt::Display for SDLError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      SDLError::Window(ref cause) => write!(f, "{}", cause),
      SDLError::IntegerOrSdl(ref cause) => write!(f, "{}", cause),
      SDLError::Emulator(ref cause) => write!(f, "{}", cause),
      SDLError::Other(ref cause) => write!(f, "{}", cause),
    }
  }
}

impl From<video::WindowBuildError> for SDLError {
  fn from(value: video::WindowBuildError) -> Self {
    SDLError::Window(value)
  }
}

impl From<IntegerOrSdlError> for SDLError {
  fn from(value: IntegerOrSdlError) -> Self {
    SDLError::IntegerOrSdl(value)
  }
}

impl From<String> for SDLError {
  fn from(value: String) -> Self {
    SDLError::Other(value)
  }
}

impl From<EmuError> for SDLError {
  fn from(value: EmuError) -> Self {
    SDLError::Emulator(value)
  }
}
