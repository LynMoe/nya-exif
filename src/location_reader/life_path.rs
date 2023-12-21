use csv::ReaderBuilder;
use simple_log::log::{error, debug};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::str;
use std::env;

use crate::location_reader::{LocationReaderBase, LocationReaderParam, LocationReaderResult};

pub struct LocationReaderLiftPath {
  param: LocationReaderParam,
  file: File,
  index: Vec<(i32, u64)>, // (timestamp, position)
}

impl LocationReaderLiftPath {
  pub fn new(param: LocationReaderParam) -> Self {
    let mut file_path;
    
    if env::consts::OS != "macos" && param.data_path.is_none() {
      error!("Location data file path(-f) required");
      panic!();
    }

    if param.data_path.is_none() {
      file_path = dirs::home_dir().unwrap();
      file_path.push("Library/Mobile Documents/iCloud~com~lifubing~lbs~stepOfLife/Documents");
    } else {
      file_path = param.data_path.clone().unwrap();
    }
    file_path.push("backUpData.csv");

    debug!("[LifePath] folder path: {:?}", file_path);

    if !file_path.exists() {
      error!("[LifePath] CSV file not exists, path {:?}", file_path);
      panic!();
    }

    let file = File::open(&file_path).unwrap();
    let mut reader = BufReader::new(file.try_clone().unwrap());
    let mut index = Vec::new();

    let mut rdr = ReaderBuilder::new()
      .has_headers(true)
      .from_reader(&mut reader);

    for result in rdr.byte_records() {
      let record = result.unwrap();
      let pos = record.position().unwrap().byte();

      let timestamp = str::from_utf8(&record[0]).unwrap().parse::<i32>().unwrap();
      index.push((timestamp, pos));
    }

    index.sort_by(|a: &(i32, u64), b| a.0.cmp(&b.0));

    LocationReaderLiftPath { param, file, index }
  }
}

impl LocationReaderBase for LocationReaderLiftPath {
  fn get_location(&mut self, timestamp: i32) -> Option<LocationReaderResult> {
    let timestamp = timestamp - self.param.time_offset;
    let pos = self.find_closest_position(timestamp);
    if pos.is_none() {
      return None;
    }
    let pos = pos.unwrap();

    self.file.seek(SeekFrom::Start(pos.0)).unwrap();
    let mut rdr = ReaderBuilder::new()
      .has_headers(false)
      .from_reader(&self.file);
    let record1 = rdr.records().next()?.unwrap();

    self.file.seek(SeekFrom::Start(pos.1)).unwrap();
    let mut rdr = ReaderBuilder::new()
      .has_headers(false)
      .from_reader(&self.file);
    let record2 = rdr.records().next()?.unwrap();

    debug!("[LifePath] search record1: {:?}", record1);
    debug!("[LifePath] search record2: {:?}", record2);

    let d1 = (record1[0].parse::<i32>().unwrap() - timestamp).abs();
    let d2 = (record2[0].parse::<i32>().unwrap() - timestamp).abs();
    let p1 = (d2 as f64) / (d1 as f64 + d2 as f64);
    let p2 = (d1 as f64) / (d1 as f64 + d2 as f64);

    let time_mid = ((record1[0].parse::<u64>().unwrap() as f64 * p1 + record2[0].parse::<u64>().unwrap() as f64 * p2)) as i32;
    let lat_mid = record1[3].parse::<f64>().unwrap() * p1 + record2[3].parse::<f64>().unwrap() * p2;
    let lon_mid = record1[2].parse::<f64>().unwrap() * p1 + record2[2].parse::<f64>().unwrap() * p2;
    let alt_mid = record1[10].parse::<f64>().unwrap() * p1 + record2[10].parse::<f64>().unwrap() * p2;
    let confidence_radius_min = record1[5].parse::<f32>().unwrap().min(record2[5].parse::<f32>().unwrap());

    debug!("[LifePath] time_mid: {}, max interval: {}", time_mid, self.param.max_interval);

    if (time_mid - timestamp).abs() > self.param.max_interval as i32 {
      return None;
    }

    Some(LocationReaderResult {
      lat: lat_mid,
      lon: lon_mid,
      alt: alt_mid,
      confidence_radius: confidence_radius_min,
      timestamp: time_mid,
    })
  }
}

impl LocationReaderLiftPath {
  fn find_closest_position(&self, timestamp: i32) -> Option<(u64, u64)> {
    if self.index.len() <= 2 {
      return None;
    }

    let mut left = 0;
    let mut right = self.index.len() - 1;

    while left < right {
      let mid = left + (right - left) / 2;
      if self.index[mid].0 < timestamp {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    if left == 0 {
      return Some((self.index[0].1, self.index[1].1));
    }

    let diff2 = (timestamp - self.index[left - 1].0).abs();
    let mut result = (left, left - 1);

    if left + 1 < self.index.len() {
      let diff3 = self.index[left + 1].0 - timestamp;
      if diff3 > diff2 {
        result = (left, left + 1);
      }
    }

    Some((self.index[result.0].1, self.index[result.1].1))
  }
}
