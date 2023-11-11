# nya-exif

<a href="README.md">中文</a> | English

![GitHub release (with filter)](https://img.shields.io/github/v/release/LynMoe/nya-exif)

## Introduction

`nya-exif` is a versatile tool designed to match and write photo GPS data into file EXIF information. It supports JPEG, PNG, and major camera manufacturers' mainstream RAW formats. Developed in Rust, this tool is compatible with all platforms.

## Features

- [x] Supports JPEG and PNG as well as mainstream RAW formats from major camera manufacturers
- [x] Multi-platform support
- [x] Supports GCJ-02 and WGS-84 coordinate systems (defaults to GCJ-02)
- [x] Automatically detects location data and converts it to the corresponding coordinate system
- [x] Multi-threaded processing

## DEMO

```shell
➜  nya-exif /path/to/image/folder/
2023-11-11 12:46:08.337698000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4100.JPG] Location updated, lat: 34.7737885, lon: 131.9007701
2023-11-11 12:46:08.394225000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4114.JPG] Location updated, lat: 34.67844170666667, lon: 131.83647663733333
2023-11-11 12:46:08.434180000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4128.JPG] Location updated, lat: 34.68192337279844, lon: 131.8327970596869
⠉ [00:00:03] [###########################>-----------------------------------------------]      54/77      (1.8s)
⠉ [00:00:03] [###########################>-----------------------------------------------]      55/77      (1.6s)
⠉ [00:00:03] [#############################>---------------------------------------------]      60/79      (1.4s)
```

## Usage

Ensure [ExifTool](https://exiftool.org/) is installed and added to PATH.

```shell
# On macOS, you can directly run the "Lifetime Footprints" to start iCloud backup.
nya-exif /path/to/images

# On other platforms, you need to copy the lifetime footprint data directory to local.
nya-exif -f /path/to/life-path/data /path/to/images

# If the ExifTool installation path is not in PATH, manually specify the executable file location.
nya-exif -b /path/to/exiftool /path/to/images

# Specify the target coordinate system, default is China's GCJ-02 coordinate system.
nya-exif -c wgs84 /path/to/images
```

> [!NOTE]
> It is recommended to run the program on local files. If running on a network drive, the speed of the program will be affected.

## ExifWriter/LocationReader Table

| Exif Writer | Description |
| --- | --- |
| [ExifTool](https://exiftool.org/) | Supports reading and writing EXIF information for most image files, supported across all platforms<br>**Installation**: Download the corresponding platform installation package from the [official website](https://exiftool.org/), install directly. After adding to PATH, this tool will automatically search for the program path. |

| Location Reader | Description |
| --- | --- |
| [一生足迹](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399) | "Life Path" is an iOS application that records user's footprints, with low power consumption and can stay in the background.<br>**Installation**：[App Store](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399) Download and install, need to enable iCloud sync<br>**Usage**: For macOS users, the program will automatically find the backup location of life's footprints in iCloud; for users of other platforms, you need to manually specify the directory (containing `backUpData.csv` file) location. |

The current program default option is `ExifTool` + `Lifetime Footprints`. After installing `ExifTool.pkg` on the macOS platform, you can directly use the default parameters to start.

## Command line parameters.

```shell
Usage: nya-exif [OPTIONS] [PATH]

Arguments:
  [PATH]
          Path to photography files

Options:
  -r, --recursive
          Turn on recursive mode

  -x, --threads <THREADS>
          Threads
          
          Number of threads to use.
          
          [default: 3]

  -w, --writer-type <WRITER_TYPE>
          Exif writer type
          
          [default: exiftool]

          Possible values:
          - exiftool: ExitTool(https://exiftool.org/)

  -b, --writer-bin-path <WRITER_BIN_PATH>
          Exif writer binary path
          
          Path to the exif writer binary.
          
          Leave it blank for the program to search automatically.

  -l, --location-reader-type <LOCATION_READER_TYPE>
          Location reader type
          
          LiftPath(一生足迹): https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399 On MacOS, the program will automatically search for Lifetime Footprint data in the user's iCloud directory. In systems other than MacOS, you need to manually specify the directory.
          
          [default: life-path]

          Possible values:
          - life-path: LifePath(一生足迹)

  -f, --location-file-path <LOCATION_FILE_PATH>
          Location file path
          
          The corresponding location reader's data directory path. Leave it blank for the program to search automatically.

  -i, --location-max-interval <LOCATION_MAX_INTERVAL>
          Location max interval in seconds
          
          Specifies the maximum time interval for location data near the photo time.
          
          If the difference between the timestamp of the location data and the photo exceeds this value, the location data will not be written.
          
          [default: 600]

  -c, --location-coordinate-target <LOCATION_COORDINATE_TARGET>
          Location GPS coordinate convert target
          
          Specifies the target coordinate system for converting GPS coordinates. Default is Auto-detect.

          Possible values:
          - wgs84: Global coordinate system
          - gcj02: China coordinate system

  -o, --overwrite-original
          Overwrite original file

  -t, --time-offset <TIME_OFFSET>
          Time offset in seconds
          
          Used for situations where the camera time is inconsistent with real time.
          
          E.g. the camera time is 1 hour ahead of real time, then fill in 3600 here.
          
          [default: 0]

  -d, --debug
          Turn on debug mode

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Contributing

If there are other location recording software that need support, feel free to submit a PR or attach detailed data information to submit an Issue.

If you encounter files that ExifTool cannot handle, please attach the file and submit an Issue.

For ExifWriter and LocationReader, please refer to the existing implementations in the `src/exif_writer` and `src/location_reader` directories. After implementing the corresponding Trait, register it in `src/core/app.rs`.

> [!IMPORTANT] 
> The latitude and longitude returned by the Location Reader should be in the Earth coordinate system (WGS84), this tool will convert according to the coordinate system selected by the user.

## License

[MIT](LICENSE) ©[Lyn](mailto://i@lyn.moe)
