# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

## [0.3.2] - 2024-10-7

### Added

* 添加日志，记录文件写入状态

## [0.4.0] - 2024-10-7

### Fixed

* 尝试创建父目录
* 修改 ConfigCheckStrategy: 如果文件为空（或为新文件），则始终写入

## [0.3.0] - 2024-9-26

### Added

* 添加 `random` 和 `md5` 策略

## [0.2.0] - 2024-9-26

### Changed

* 拆分 `CheckStrategy` 逻辑到各个子类，便于扩展

## [0.1.1] - 2024-9-5

### Added

* 添加 `First64Bytes` 策略

## [0.1.0] - 2024-9-5

### Added

* 实现基本入口方法 `release_file` 和 `release_file_with_check`
