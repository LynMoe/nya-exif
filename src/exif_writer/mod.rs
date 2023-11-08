use std::path::PathBuf;

pub mod exiftool;

pub struct ExifWriterParam {
  pub binary_path: Option<PathBuf>,
}

pub trait ExifWriterBase {
  fn read_timestamp(&mut self, path: &str) -> i64;
  fn write_location(&mut self, path: &str, lat: f64, lon: f64, alt: f64);
  fn change_timezone_offset(&mut self, path: &str, timezone_offset: &str);
}