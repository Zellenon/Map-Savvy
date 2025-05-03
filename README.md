# 🗺 Map-Savvy - Procedural World Map Generator 
**Blazing-fast fractal-based Rust-powered terrain generation with an Egui interface**  
[![Rust](https://img.shields.io/badge/Rust-1.81%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache-blue)](https://opensource.org/licenses/MIT)
[![Performance](https://img.shields.io/badge/Speed-10x_C_implementation-brightgreen)](https://www.lysator.liu.se/~johol/fwmg/howisitdone.html)

## 🌍 Overview
MapSavvy implements the **Voss Fractal Faulting Algorithm**[[1]](https://www.lysator.liu.se/~johol/fwmg/howisitdone.html) to create realistic Mercator-projection world maps through GPU-accelerated Rust computation. Key features:

- **10x speed boost** over original C implementation through:
  - Multithreaded fault line generation (Rayon parallel iterators)
  - Zero-copy texture handling with Egui integration 
- **Real-time parameter control** via Egui's immediate mode GUI:
  - Interactive seed control
- **Export capabilities**:
  - 16K+ PNG/JPG map renders
  - Heightmap data (CSV/RAW)

## 🚀 Installation
```bash
# Clone repository
git clone https://github.com/zellenon/mapsavvy.git
cd mapsavvy

# Build with optimizations (requires Rust 1.81+)
cargo build --release

# Run application
cargo run --release
```


# License
Dual-licensed under either:

    MIT License (LICENSE-MIT)

    Apache License 2.0 (LICENSE-APACHE)
