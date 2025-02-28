# Pico W Rust Projects & Utilities

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Collection of embedded Rust projects and utilities for Raspberry Pi Pico W development. Contains examples and reusable components for hardware interaction using the Embassy framework.

## 🛠️ Technology Stack
- **Microcontroller**: Raspberry Pi Pico W  
- **Language**: Rust (nightly)  
- **Framework**: Embassy  
- **HAL**: rp2040-hal  
- **WiFi**: cyw43-driver  
- **Flashing**: elf2uf2-rs  

## 🔧 Prerequisites
```bash
rustup default nightly
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs
```
## 🚀 How to Run
```bash
# 1. First step: Hold BOOTSEL button while connecting USB

# 2. Then navigate to project and run:
cd project_directory
cargo run --release
```
