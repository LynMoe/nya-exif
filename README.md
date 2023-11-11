# nya-exif

中文 | <a href="README_en.md">English</a>

![GitHub release (with filter)](https://img.shields.io/github/v/release/LynMoe/nya-exif)

## 介绍

`nya-exif` 是一个用于匹配照片 GPS 信息, 并写入文件 EXIF 信息的工具, 支持 JPEG 和 PNG 及各大相机厂商的主流RAW格式. 本工具基于 Rust 编写, 支持全平台使用

## Features

- [x] 支持 JPEG 和 PNG 及各大相机厂商的主流RAW格式
- [x] 全平台支持
- [x] 支持国策局 GCJ-02 和 WGS-84 坐标系 (解决国内坐标漂移问题)
- [x] 自动检测国内外位置, 自动转换为对应坐标系

## DEMO

```shell
➜  nya-exif /path/to/image/folder/
2023-11-11 12:46:08.337698000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4100.JPG] Location updated, lat: 34.7737885, lon: 131.9007701
2023-11-11 12:46:08.394225000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4114.JPG] Location updated, lat: 34.67844170666667, lon: 131.83647663733333
2023-11-11 12:46:08.434180000 [INFO] <nya_exif::core::app:130>:[20230908-_MGL4128.JPG] Location updated, lat: 34.68192337279844, lon: 131.8327970596869
⠂ [00:00:04] [###########################>-----------------------------------------------]      93/233     (6.7s)
```

## 使用

确保已安装 [ExifTool](https://exiftool.org/), 并添加至 PATH

```shell
# macOS 下, 一生足迹启动 iCloud 云备份, 可直接运行
nya-exif .

# 其他平台下, 需要将一生足迹数据目录拷贝至本地
nya-exif -f /path/to/life-path/data /path/to/images

# 若 ExifTool 安装路径不在 PATH 中, 手动指定可执行文件位置
nya-exif -b /path/to/exiftool /path/to/images

# 指定目标坐标系, 默认为自动检测, 如果在边境线附近需要手动指定
nya-exif -c wgs84 /path/to/images
```

> [!NOTE]  
> 推荐在本地文件上运行程序, 若在网络盘上运行, 会影响程序速度

## ExifWriter/LocationReader 支持情况

| Exif Writer | 描述 |
| --- | --- |
| [ExifTool](https://exiftool.org/) | 支持大多数图片文件的EXIF信息读写, 全平台支持<br>**安装**: [官网](https://exiftool.org/)下载对应平台安装包, 直接安装即可, 添加至PATH后本工具会自动查找程序路径; 若未添加到 PATH, 需要手动指定 `-b` 选项到程序的二进制文件绝对路径 |

| Location Reader | 描述 |
| --- | --- |
| [一生足迹](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399) | 一生足迹是一款 iOS 端记录用户足迹的应用, 耗电量较低, 可常驻后台<br>**安装:** [App Store](https://apps.apple.com/us/app/footprint-record-lifes-path/id1225520399)下载安装即可, 需要开启 iCloud 同步<br>**使用:** 对于 macOS 用户, 程序会自动查找一生足迹在 iCloud 中的备份位置; 对于其他平台用户, 需要手动指定目录(含`backUpData.csv`文件)的位置<br>**注意:** 该备份文件的同步不是很及时, 如果拍摄时间较新需要在 App 中手动到处数据文件加载 |

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

若有其他的位置记录软件需要支持, 欢迎提交 PR 或附上详细数据信息提交 Issue

若遇到 ExifTool 无法处理的文件, 请附上文件信息提交 Issue

对于 ExifWriter 和 LocationReader, 请参考 `src/exif_writer` 和 `src/location_reader` 目录下已有的实现, 实现对应的 Trait 后在 `src/core/app.rs` 中注册即可

> [!IMPORTANT]  
> Location Reader返回的经纬度应该为地球坐标系(WGS84), 本工具会根据用户选择的坐标系进行转换

## Declaimer

本工具中附带的 `GCJ-02` 范围数据仅用于粗略地理位置判断, 不具有任何立场和政治倾向, 请勿用于其他用途 

## License

[MIT](LICENSE) ©[Lyn](mailto://i@lyn.moe)
