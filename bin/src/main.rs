pub mod prelude {
  #[cfg(not(feature = "vita"))]
  pub use sdl2;
  #[cfg(feature = "vita")]
  pub use sdl2_vita as sdl2;
}

use nes::core::cartridge::Cartridge;
use prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod err;

pub fn main() -> err::Result<()> {
  let rom = match std::env::args().nth(1) {
    Some(file) => nes::util::io::read(file)?,
    None => Err("Failed to get file path.".to_string())?,
  };

  dbg!(&rom);
  let _cartridge = Cartridge::new(rom);

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
