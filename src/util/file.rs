use std::fs;
use std::io;
use std::path::Path;

use simple_log::log::debug;

pub fn read_dir_files(dir: &Path, full_path: bool, recursive: bool) -> io::Result<Vec<String>> {
  let exts = vec!["jpg", "jpeg", "png", "cr3", "dng", "heic", "RW2", "raw", "raf", "arw", "arq", "nef", "nrw"];

  debug!("[Util readDir] Reading dir: {:?}", dir);

  let mut files = Vec::new();

  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() && recursive {
        files.extend(read_dir_files(&path, recursive, full_path)?);
      } else if path.is_file() {
        if path.file_name().unwrap().to_string_lossy().starts_with(".") {
          continue;
        }
        let ext = path.extension().unwrap_or("".to_owned().as_ref()).to_string_lossy().to_lowercase();
        if !exts.contains(&ext.as_ref()) {
          debug!("[Util readDir] Skipping file: {:?}", path);
          continue;
        }

        files.push(if full_path {
          path.to_string_lossy().to_string()
        } else {
          path.file_name().unwrap().to_string_lossy().to_string()
        });
      }
    }
  }

  Ok(files)
}
