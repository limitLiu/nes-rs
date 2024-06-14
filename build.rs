fn main() {
  let target = std::env::var("TARGET").unwrap();
  if target == "armv7-sony-vita-newlibeabihf" {
    println!("cargo:rustc-cfg=feature=\"vita\"");
  } else {
    println!("cargo:rustc-cfg=feature=\"default\"");
  }
}
