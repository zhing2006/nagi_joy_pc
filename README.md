# nagi-joy-pc
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md)

## Introduction
This is a server program based on vJoy that needs to be used together with [nagi-joy-esp32](https://github.com/zhing2006/nagi_joy_esp32). It receives controller data from `nagi-joy-esp32` via the UDP protocol and forwards the data to vJoy according to the configuration file, implementing various functions of the game controller.

## Features
- [x] Button
- [x] Axis
- [ ] POV Hat

## Configure
The configuration file is located at `conf/config.toml`, and its definition is as follows.

```
[service] // Define the UDP listening IP and port
  host = "0.0.0.0"
  port = 8888

[joystick]  // Specify which vJoy controller to send data to (1 is the first controller)
  index = 1

[[joystick.buttons]]  // Configure specific buttons, can invert button state
  index = 9
  inverted = true

[[joystick.axes]] // Configure specific axes, set min, max values, and whether to invert the axis
  index = 0
  inverted = true
  max = 3280
  min = 4
```