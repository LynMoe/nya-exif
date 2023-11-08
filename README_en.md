# nya-exif

<a href="README.md">中文</a> | English

## Introduction

`nya-exif` is a versatile tool designed to match and write photo GPS data into file EXIF information. It supports JPEG, PNG, and major camera manufacturers' mainstream RAW formats. Developed in Rust, this tool is compatible with all platforms.

## DEMO

```shell
➜  nya-exif /path/to/image/folder/
2023-11-08 15:57:30.830962000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4076.JPG
2023-11-08 15:57:30.931190000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4062.JPG
2023-11-08 15:57:30.967376000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4089.JPG
⠂ [00:00:04] [###########################>-----------------------------------------------]      93/233     (6.7s)
```

## Usage

```shell
# On macOS, you can directly run the "Lifetime Footprints" to start iCloud backup.
nya-exif /path/to/images

# On other platforms, you need to copy the lifetime footprint data directory to local.
nya-exif -f /path/to/life-path/data /path/to/images

# If the ExifTool installation path is not in PATH, manually specify the executable file location.
nya-exif -b /path/to/exiftool /path/to/images
```

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

## License

[MIT](LICENSE) ©[Lyn](mailto://i@lyn.moe)