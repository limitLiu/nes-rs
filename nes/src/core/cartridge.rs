#[derive(Debug, duplicated::New)]
pub struct ROM {
  pub prg: Vec<u8>,
  pub chr: Vec<u8>,
  pub mapper: u8,
}

#[derive(Debug, duplicated::New)]
pub struct Cartridge {
  pub rom: ROM,
}
