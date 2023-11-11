use crate::exif_writer::{exiftool::ExifWriterExifTool, ExifWriterBase, ExifWriterParam};
use crate::location_reader::{life_path, LocationReaderBase, LocationReaderParam};
use crate::util::{file, location as lc};
use clap::ValueEnum;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use simple_log::log::{info, warn};
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::thread;
use undrift_gps::wgs_to_gcj;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ExifWriterType {
  /// ExitTool(https://exiftool.org/)
  Exiftool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LocationReaderType {
  /// LifePath(一生足迹)
  LifePath,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LocationGpsCoordinateTarget {
  /// Global coordinate system
  WGS84,
  /// China coordinate system
  GCJ02,
}

#[derive(Debug, Clone)]
pub struct AppParams {
  pub operate_dir: Vec<PathBuf>,
  pub recursive: bool,
  pub thread_count: u32,
  pub writer_type: ExifWriterType,
  pub writer_bin_path: Option<PathBuf>,
  pub location_reader_type: LocationReaderType,
  pub location_file_path: Option<PathBuf>,
  pub location_max_interval: u32,
  pub location_gps_coordinate_target: Option<LocationGpsCoordinateTarget>,
  pub overwrite_original: bool,
  pub time_offset: i32,
}

pub fn run(params: AppParams) {
  let mut fi = vec![];
  for dir in params.operate_dir.clone() {
    let fi_ = file::read_dir_files(dir.as_ref(), true, true).unwrap();
    fi.extend(fi_);
  }

  // split fi in params.thread_count parts
  let mut fi_parts = vec![];
  let mut now_state = 0;
  let mut now_part = 0;
  let part_size = fi.len() / params.thread_count as usize;
  for file in fi {
    if now_state == 0 {
      fi_parts.push(vec![]);
    }

    fi_parts[now_part].push(file);

    now_state += 1;
    if now_state == part_size && now_part + 1 < params.thread_count as usize {
      now_state = 0;
      now_part += 1;
    }
  }

  let m = MultiProgress::new();
  let sty = ProgressStyle::with_template(
    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})",
  )
  .unwrap()
  .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
  })
  .progress_chars("#>-");

  let mut threads = vec![];
  for fi in fi_parts {
    let count = fi.len();
    let pb = m.add(ProgressBar::new(count as u64));
    pb.set_style(sty.clone());

    let params = params.clone();
    threads.push(thread::spawn(move || {
      let exif_param = ExifWriterParam {
        binary_path: params.writer_bin_path.clone(),
      };

      let mut exiftool: Box<dyn ExifWriterBase>;
      match params.writer_type {
        ExifWriterType::Exiftool => {
          exiftool = Box::new(ExifWriterExifTool::new(exif_param));
        }
      }

      let location_param = LocationReaderParam {
        data_path: params.location_file_path.clone(),
        time_offset: params.time_offset.clone(),
        max_interval: params.location_max_interval.clone(),
      };

      let mut location_reader: Box<dyn LocationReaderBase>;
      match params.location_reader_type {
        LocationReaderType::LifePath => {
          location_reader = Box::new(life_path::LocationReaderLiftPath::new(location_param));
        }
      }

      let mut now_state = 0;
      for file in fi {
        now_state += 1;

        let filename = file.as_str();
        let time = exiftool.read_timestamp(filename);
        let location = location_reader.get_location(time as i32);

        if location.is_some() {
          let mut location = location.unwrap();

          if params.location_gps_coordinate_target.is_some() {
            match params.location_gps_coordinate_target.unwrap() {
              LocationGpsCoordinateTarget::GCJ02 => {
                let (lat, lon) = wgs_to_gcj(location.lat, location.lon);
                location.lat = lat;
                location.lon = lon;
              }
              _ => {}
            }
          } else {
            let result = lc::is_point_in_gcj_region((location.lat, location.lon));

            if result {
              let (lat, lon) = wgs_to_gcj(location.lat, location.lon);
              location.lat = lat;
              location.lon = lon;
            }
          }

          exiftool.write_location(filename, location.lat, location.lon, location.alt);

          pb.suspend(|| {
            let filename = Path::new(filename).file_name().unwrap().to_str().unwrap();
            info!(
              "[{}] Location updated, lat: {}, lon: {}",
              filename, location.lat, location.lon
            );
          });
        } else {
          pb.suspend(|| {
            let filename = Path::new(filename).file_name().unwrap().to_str().unwrap();
            warn!("[{}] No matching location, timestamp {}", filename, time);
          });
        }

        pb.set_position(now_state);
      }
    }));
  }

  for thread in threads {
    let _ = thread.join();
  }
  m.clear().unwrap();
  info!("Finished");

  // pb.finish_with_message("Finished");
}
