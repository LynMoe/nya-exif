use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, ChildStdin, ChildStdout, Stdio};
use chrono::DateTime;
use simple_log::log::debug;
use which::which;

use crate::exif_writer::{ExifWriterBase, ExifWriterParam};

pub struct ExifWriterExifTool {
  stdin: Option<ChildStdin>,
  stdout: Option<BufReader<ChildStdout>>,
}

impl ExifWriterExifTool {
  pub fn new(param: ExifWriterParam) -> Self {
    let path: PathBuf;

    if param.binary_path.is_none() {
      // auto detect
      path = match os_info::get().os_type() {
        _ => {
          which("exiftool").unwrap()
        }
      };
    } else {
      path = PathBuf::from(param.binary_path.unwrap());
    }

    let mut child = Command::new(&path)
        .arg("-stay_open")
        .arg("true")
        .arg("-@")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start exiftool");

    let stdin = child.stdin.take().unwrap();
    let stdout = BufReader::new(child.stdout.take().unwrap());

    Self { stdin: Some(stdin), stdout: Some(stdout) }
  }

  fn execute_command(&mut self, filename: &str, args: &[String]) -> String {
    let stdin = self.stdin.as_mut().unwrap();
    let stdout = self.stdout.as_mut().unwrap();

    writeln!(stdin, "{}", filename).unwrap();
    for arg in args {
        writeln!(stdin, "{}", arg).unwrap();
    }
    writeln!(stdin, "-execute").unwrap();

    let mut result = String::new();
    loop {
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        if line.trim() == "{ready}" {
            break;
        }
        result.push_str(&line);
    }

    debug!("Exiftool arg: {:?}, result: {}", args, result);

    result
  }
}

impl ExifWriterBase for ExifWriterExifTool {
  fn read_timestamp(&mut self, path: &str) -> i64 {
    let args = [
      "-createDate".to_string(),
      "-d".to_string(),
      "%Y:%m:%d %H:%M:%S %z".to_string(),
    ];

    // let output = 
    let result = self.execute_command(path, &args);
    let lines: Vec<&str> = result.split("\n").map(|line| {
      let mut ll = line.split(": ");

      if line.len() == 0 {
        return "";
      }

      ll.next();
      ll.next().unwrap().trim()
    }).collect();

    let time_string = lines[0].to_owned();
    let dt = DateTime::parse_from_str(&time_string, "%Y:%m:%d %H:%M:%S %z")
        .expect("Failed to parse date time string");

    debug!("Exiftool read timestamp: {}, time_string: {}", dt.timestamp(), time_string);

    dt.timestamp()
  }

  fn write_location(&mut self, path: &str, lat: f64, lon: f64, alt: f64) {
    let args = [
      "-GPSLatitude=".to_string() + &lat.to_string(),
      "-GPSLongitude=".to_string() + &lon.to_string(),
      "-GPSLatitudeRef=N".to_string(),
      "-GPSLongitudeRef=E".to_string(),
      "-GPSAltitude=".to_string() + &alt.to_string(),
      "-GPSAltitudeRef=0".to_string(),
      "-overwrite_original".to_string(),
    ];

    // let output = 
    self.execute_command(path, &args);
  }

  fn change_timezone_offset(&mut self, path: &str, timezone_offset: &str) {
    let args = [
      "-EXIF:OffsetTime*=".to_string() + timezone_offset, // eg, +2:00
      "-overwrite_original".to_string(),
    ];

    // let output = 
    self.execute_command(path, &args);
  }
}
