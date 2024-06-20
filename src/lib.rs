pub mod prelude {
  #[cfg(not(feature = "vita"))]
  pub use sdl2;
  #[cfg(feature = "vita")]
  pub use sdl2_vita as sdl2;
}

pub mod err;
