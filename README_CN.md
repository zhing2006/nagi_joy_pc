# nagi-joy-pc
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md)

## 介绍
这是一个需要配合[nagi-joy-esp32](https://github.com/zhing2006/nagi_joy_esp32)一同使用的基于vJoy的服务端程序。通过UDP协议，接收来自`nagi-joy-esp32`的控制器数据，并根据配置文件将数据转发到vJoy。实现游戏控制器的各种功能。

## 功能
- [x] 按钮
- [x] 轴
- [ ] 苦力帽

## 配置
配置文件位于`conf/config.toml`，其定义如下。

```
[service] // 定义UDP监听IP和端口
  host = "0.0.0.0"
  port = 8888

[joystick]  // 数据发送到vJoy的几号控制器（1是第一个控制器）
  index = 1

[[joystick.buttons]]  // 对特定按钮进行配置，可以反转按钮状态
  index = 9
  inverted = true

[[joystick.axes]] // 对特定轴进行配置，设置最小值、最大值和是否反转轴
  index = 0
  inverted = true
  max = 3280
  min = 4
```