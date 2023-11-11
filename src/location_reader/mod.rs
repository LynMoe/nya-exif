use std::path::PathBuf;
pub mod life_path;

pub struct LocationReaderParam {
  pub data_path: Option<PathBuf>,
  pub time_offset: i32,
  pub max_interval: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationReaderResult {
  pub lat: f64,
  pub lon: f64,
  pub alt: f64,
  pub confidence_radius: f32, 
  pub timestamp: i32,
}

pub trait LocationReaderBase {
  // fn new(param: ExifWriterParam) -> Self;
  fn get_location(&mut self, timestamp: i32) -> Option<LocationReaderResult>;
}
