# nya-exif

中文 | <a href="README_en.md">English</a>

## 介绍

`nya-exif` 是一个用于匹配照片 GPS 信息, 并写入文件 EXIF 信息的工具, 支持 JPEG 和 PNG 及各大相机厂商的主流RAW格式. 本工具基于 Rust 编写, 支持全平台使用

## DEMO

```shell
➜  nya-exif /path/to/image/folder/
2023-11-08 15:57:30.830962000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4076.JPG
2023-11-08 15:57:30.931190000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4062.JPG
2023-11-08 15:57:30.967376000 [INFO] <nya_exif::core::app:84>:Updating location for 20230908-_MGL4089.JPG
⠂ [00:00:04] [###########################>-----------------------------------------------]      93/233     (6.7s)
```

## 使用

```shell
# macOS 下, 一生足迹启动 iCloud 云备份, 可直接运行
nya-exif /path/to/images

# 其他平台下, 需要将一生足迹数据目录拷贝至本地
nya-exif -f /path/to/life-path/data /path/to/images

# 若 ExifTool 安装路径不在 PATH 中, 手动指定可执行文件位置
nya-exif -b /path/to/exiftool /path/to/images
```

## ExifWriter/LocationReader 支持情况

| Exif Writer | 描述 |
| --- | --- |
| [ExifTool](https://exiftool.org/) | 支持大多数图片文件的EXIF信息读写, 全平台支持<br>**安装**: [官网](https://exiftool.org/)下载对应平台安装包, 直接安装即可, 添加至PATH后本工具会自动查找程序路径; 若未添加到 PATH, 需要手动指定 `-b` 选项到程序的二进制文件绝对路径 |

| Location Reader | 描述 |
| --- | --- |
| [一生足迹](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399) | 一生足迹是一款 iOS 端记录用户足迹的应用, 耗电量较低, 可常驻后台<br>**安装**: [App Store](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399)下载安装即可, 需要开启 iCloud 同步<br>**使用**：对于 macOS 用户, 程序会自动查找一生足迹在 iCloud 中的备份位置; 对于其他平台用户, 需要手动指定目录(含`backUpData.csv`文件)的位置 |

目前程序默认选项为 `ExifTool` + `一生足迹`, 在 macOS 平台安装 `ExifTool.pkg` 后可直接使用默认参数启动工具

## 命令行参数

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

若有其他的位置记录软件需要支持, 欢迎提交 PR 或附上详细数据信息提交 Issue

若遇到 ExifTool 无法处理的文件, 请附上文件信息提交 Issue

对于 ExifWriter 和 LocationReader, 请参考 `src/exif_writer` 和 `src/location_reader` 目录下已有的实现, 实现对应的 Trait 后在 `src/core/app.rs` 中注册即可

## License

[MIT](LICENSE) ©[Lyn](mailto://i@lyn.moe)
