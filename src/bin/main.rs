use nes::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> bin_err::Result<()> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("NES Emulator", 960, 544)
    .position_centered()
    .build()?;

  let mut canvas = window.into_canvas().build()?;

  canvas.set_draw_color(Color::RGB(51, 100, 100));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump()?;

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }

    canvas.clear();
    canvas.present();
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
  }

  Ok(())
}

mod bin_err {
  use nes::prelude::*;
  use sdl2::{video, IntegerOrSdlError};

  pub type Result<T> = std::result::Result<T, SDLError>;

  #[derive(Debug)]
  pub enum SDLError {
    Window(video::WindowBuildError),
    IntegerOrSdl(IntegerOrSdlError),
    Normal(String),
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
        SDLError::Normal(ref cause) => write!(f, "{}", cause),
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
      SDLError::Normal(value)
    }
  }
}
