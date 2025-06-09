# 🎮 Cursor360

> Convert mouse and keyboard inputs into a virtual Xbox 360 controller using ViGEm — for enhanced game control, emulators, and precision aim assistance.

![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Status](https://img.shields.io/badge/status-Prototype-yellow.svg)

---

## 🧩 About

**Cursor360** is a high-performance tool that transforms mouse movements and keyboard actions into a fully functional **virtual Xbox 360 controller**, using the [ViGEm](https://vigem.org/) (Virtual Gamepad Emulation Framework).  
Perfect for:

- Emulators (like **Xenia**)
- Games without native mouse+keyboard support
- Custom input remapping
- Experimental camera control mechanics

---

## ✨ Features
**Only tested on Gears of War**
- 🔁 **Realtime Input Mapping** — Converts keyboard & mouse into controller inputs instantly (need better and more implementations)
- 🎯 **Smooth Camera Movement** — Customizable friction, sensitivity & centering (needs some fix)
- 🕹️ **Supports Full Gamepad Emulation** — Thumbsticks, triggers, buttons
- 🔒 **Locks and recenters the cursor** — Ideal for FPS-style camera control
- 🛠️ **Flexible & Easy to Modify** — Built in Rust with performance in mind

---

## 🚀 Getting Started

### ✅ Requirements

- [Rust](https://www.rust-lang.org/) installed
- ViGEmBus driver: [Download](https://vigem.org/projects/ViGEm/ViGEm-Bus-Driver/)


### 🧪 Run

```bash
git clone https://github.com/yourusername/Cursor360.git
cd Cursor360
cargo run 

-Stop it with Ctrl + C

```

### Run Build

Execute .exe who's inside target/release
