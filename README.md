# 🧱 mod_menu: Kali Linux GUI Terminal Emulator for Windows

[![Rust](https://img.shields.io/badge/Rust-Stable%20%7C%20Nightly-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows)](https://github.com/AaYuSh11233/mod_menu)

> **A full-featured, visually accurate Kali Linux terminal UI emulator built with Rust + egui/eframe — designed to feel like home for ethical hackers, developers, and students on Windows.**

---

## 🎯 Project Goal

To **faithfully replicate the look and feel of the Kali GNOME terminal** inside a standalone Windows GUI app — while **executing real CMD commands** and **simulating popular Linux shell utilities**.

A hybrid terminal for those who love Linux's aesthetic and power but operate in a Windows environment.

---

## 🖼️ Screenshots

> ![demo](assets/demo.png)  
> *Exact reproduction of Kali's red-yellow-green UI elements, green-on-black text, and prompt styling.*

---

## 🔧 Features

| Category         | Feature                                                                 |
|------------------|-------------------------------------------------------------------------|
| 🎨 UI Design      | Custom top bar with **RGB circular buttons** (close, minimize, maximize)|
| ⌨️ Input Style     | Prompt-style input: `┌──(nInjaOS)-[~]` and `└─$`                         |
| 🖋️ Fonts          | `JetBrainsMono` for crisp terminal aesthetics                           |
| 🌌 Color Theme    | Kali-style green on black (low brightness, monospaced text)             |
| 🧠 Command Engine | Real **Windows CMD** integration (`std::process::Command`)               |
| 🐧 Fake Linux Cmd | Simulates `ls`, `clear`, `sudo`, `apt`                                  |
| 🔁 Scroll Buffer  | Full terminal-style output with scrolling & preserved history            |
| 🔒 Focus Control  | Always-on input focus, just like a real terminal                         |

---

## 🛠️ Getting Started

### ⚙️ Prerequisites

| Tool                 | Required |
|----------------------|----------|
| [Rustup](https://rustup.rs)     | ✅ |
| MSVC Build Tools (`cl.exe`, `link.exe`) | ✅ |
| Windows 10 SDK (latest)         | ✅ |
| Visual Studio 2022 Build Tools  | ✅ (C++ + Desktop Dev) |

---

### 🧪 Installation Steps

```bash
git clone https://github.com/AaYuSh11233/mod_menu.git
cd mod_menu
rustup default nightly
cargo run

```

---

## 🤝 Contributions

Contributions are welcome! If you have ideas for improvements, bug fixes, or new features, feel free to fork the repository and submit a pull request. Please make sure to follow the existing code style and include clear commit messages. For major changes, open an issue first to discuss what you would like to change.

---

## 🐞 Issues

If you encounter any bugs or have suggestions for enhancements, please open an issue on the [GitHub Issues page](https://github.com/AaYuSh11233/mod_menu/issues). Provide as much detail as possible, including steps to reproduce the problem, screenshots, and your environment details.

---

## 🚀 Future Overview

- Improved Linux command emulation (more commands, better accuracy)
- Customizable themes and fonts
- Tabbed terminal sessions
- Plugin system for user extensions
- Cross-platform support (Linux, macOS)
- Performance optimizations and reduced memory usage

Stay tuned for more updates and features!
