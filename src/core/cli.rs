use clap::Parser;
use std::path::PathBuf;
use simple_log::log::{debug, error};
use simple_log::LogConfigBuilder;

use crate::core::app::{self, ExifWriterType, LocationReaderType, LocationGpsCoordinateTarget};

#[derive(Parser)]
#[command(name = "nya-exif")]
#[command(author = "Lyn <i@lyn.moe>")]
#[command(version)]
struct Cli {
    /// Path to photography files
    path: Vec<String>,

    /// Turn on recursive mode
    #[arg(short, long, default_value_t = true)]
    recursive: bool,

    /// Threads
    /// 
    /// Number of threads to use.
    #[arg(short = 'x', long, default_value_t = 3)]
    threads: u32,

    /// Exif writer type
    #[arg(short = 'w', long, value_enum, default_value_t = ExifWriterType::Exiftool)]
    writer_type: ExifWriterType,

    /// Exif writer binary path
    /// 
    /// Path to the exif writer binary.
    /// 
    /// Leave it blank for the program to search automatically.
    #[arg(short = 'b', long)]
    writer_bin_path: Option<String>,

    /// Location reader type
    /// 
    /// LiftPath(一生足迹):
    ///   https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399
    ///   On MacOS, the program will automatically search for Lifetime Footprint data in the user's iCloud directory.
    ///   In systems other than MacOS, you need to manually specify the directory.
    #[arg(short = 'l', long, value_enum, default_value_t = LocationReaderType::LifePath)]
    location_reader_type: LocationReaderType,

    /// Location file path
    /// 
    /// The corresponding location reader's data directory path. Leave it blank for the program to search automatically.
    #[arg(short = 'f', long)]
    location_file_path: Option<String>,

    /// Location max interval in seconds
    /// 
    /// Specifies the maximum time interval for location data near the photo time.
    /// 
    /// If the difference between the timestamp of the location data and the photo exceeds this value, the location data will not be written.
    #[arg(short = 'i', long, default_value_t = 600)]
    location_max_interval: u32,

    /// Location GPS coordinate convert target
    /// 
    /// Specifies the target coordinate system for converting GPS coordinates. Default is Auto-detect.
    #[arg(short = 'c', long, value_enum)]
    location_coordinate_target: Option<LocationGpsCoordinateTarget>,

    /// Overwrite original file
    #[arg(short, long, default_value_t = true)]
    overwrite_original: bool,

    /// Time offset in seconds
    /// 
    /// Used for situations where the camera time is inconsistent with real time.
    /// 
    /// E.g. the camera time is 1 hour ahead of real time, then fill in 3600 here.
    #[arg(short, long, default_value_t = 0)]
    time_offset: i32,

    /// Turn on debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

pub fn run() {
    let cli = Cli::parse();

    let mut param = app::AppParams {
      operate_dir: Vec::new(),
      recursive: true,
      thread_count: 3,
      writer_type: app::ExifWriterType::Exiftool,
      writer_bin_path: None,
      location_reader_type: app::LocationReaderType::LifePath,
      location_file_path: None,
      location_max_interval: 1800,
      location_gps_coordinate_target: None,
      overwrite_original: false,
      time_offset: 0,
    };

    let config = LogConfigBuilder::builder()
      .level(if cli.debug { "debug" } else { "info" })
      .path("./nya-exif.log")
      .time_format("%y-%m-%d %H:%M:%S")
      .build();
    simple_log::new(config).expect("Failed to init log");

    if cli.path.len() > 0 {
      debug!("[Arg] Value for path: {:?}", cli.path);
      param.operate_dir = cli.path.iter().map(|x| PathBuf::from(x)).collect();
    } else {
      error!("No path specified");
      std::process::exit(1);
    }

    debug!("[Arg] Value for recursive: {}", cli.recursive);
    param.recursive = cli.recursive;

    debug!("[Arg] Value for threads: {}", cli.threads);
    param.thread_count = cli.threads;

    debug!("[Arg] Value for writer_type: {:?}", cli.writer_type);
    param.writer_type = cli.writer_type;

    if let Some(writer_bin_path) = cli.writer_bin_path {
      debug!("[Arg] Value for writer_bin_path: {}", writer_bin_path);
      param.writer_bin_path = Some(PathBuf::from(writer_bin_path));
    }

    debug!("[Arg] Value for location_reader_type: {:?}", cli.location_reader_type);
    param.location_reader_type = cli.location_reader_type;

    if let Some(location_file_path) = cli.location_file_path {
      debug!("[Arg] Value for location_file_path: {}", location_file_path);
      param.location_file_path = Some(PathBuf::from(location_file_path));
    }

    debug!("[Arg] Value for location_max_interval: {}", cli.location_max_interval);
    param.location_max_interval = cli.location_max_interval;

    if let Some(location_coordinate_target) = cli.location_coordinate_target {
      debug!("[Arg] Value for location_gps_coordinate_target: {:?}", location_coordinate_target);
      param.location_gps_coordinate_target = Some(location_coordinate_target);
    }

    debug!("[Arg] Value for overwrite_original: {}", cli.overwrite_original);
    param.overwrite_original = cli.overwrite_original;

    debug!("[Arg] Value for time_offset: {}", cli.time_offset);
    param.time_offset = cli.time_offset;

    debug!("[Arg] Value for app params: {:?}", param);

    app::run(param);
}