use crate::core::cartridge::ROM;

use super::err;
use std::fs;
use std::path::Path;

/// NES 1A
const INES: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

const PRG_PAGE: usize = 16 * 1024;
const CHR_PAGE: usize = 8 * 1024;

pub fn read<P: AsRef<Path>>(path: P) -> err::Result<ROM> {
  let buffer = fs::read(path)?;
  if buffer.is_empty() {
    Err("Failed to read file.".to_string())?;
  }
  let format = &buffer[0..4];
  if format != INES {
    Err("File is not nes file.".to_string())?;
  }
  let prg_size = buffer[4] as usize * PRG_PAGE;
  let chr_size = buffer[5] as usize * CHR_PAGE;
  let mapper = (buffer[7] & 0b1111_0000) | (buffer[6] >> 4);
  // skip_trainer: buffer[6] & 0b0100 != 0
  let prg_start = 16 + if buffer[6] & 0b0100 != 0 { 512 } else { 0 };
  let chr_start = prg_start + prg_size;
  let rom = ROM::new(
    buffer[prg_start..prg_start + prg_size].to_vec(),
    buffer[chr_start..chr_start + chr_size].to_vec(),
    mapper,
  );
  Ok(rom)
}
