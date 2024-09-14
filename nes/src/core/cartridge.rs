use macros::{public, New};

#[derive(Debug, New)]
pub struct ROM {
  pub prg: Vec<u8>,
  pub chr: Vec<u8>,
  pub mapper: u8,
}

#[derive(Debug, New)]
#[public]
pub struct Cartridge {
  rom: ROM,
}
