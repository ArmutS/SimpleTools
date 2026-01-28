# SimpleTools

> A modular tool collection providing quick access with Vim-like keyboard shortcuts.

SimpleTools is a desktop application designed for developers and power users, featuring an Alfred-like interface. It opens instantly with `Alt+Space` and provides navigation via vim-style keyboard shortcuts.

## ✨ Features

- 🚀 **Quick Access**: Instantly opens with `Alt+Space`
- ⌨️ **Vim-like Navigation**: Fast navigation with single-key shortcuts
- 🎯 **Modular Structure**: 12 different modules, each with 12 functions (144 tools in total)
- 🖥️ **Multi-Monitor Support**: Opens on the correct monitor based on mouse position
- 🎨 **Modern Design**: Custom UI with Catppuccin theme
- ⚡ **High Performance**: Fast operations with Rust backend
- 📱 **Responsive**: Optimized grid layouts for any screen size

## 🛠️ Technology Stack

- **Frontend**: Svelte + TypeScript
- **Backend**: Rust (Tauri v2)
- **Build Tool**: Vite

## 🐧 Linux Requirements

To run the application smoothly on Linux, the following packages must be installed:

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxtst-dev libevdev-dev
```

### Arch Linux

```bash
sudo pacman -Syu
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl gtk3 libayatana-appindicator librsvg libxtst libevdev
```

## 📦 Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Production build
npm run tauri build
```

## 🎮 Usage

### Basic Navigation

1. **Open Application**: `Alt+Space`
2. **Select Module**: Single key (e.g., `T` = Text Tools)
3. **Select Function**: Single key (e.g., `R` = Regex Tester)
4. **Go Back**: `Esc`

### Example Usage

```
Alt+Space → T → R
```

This command sequence opens the Regex Tester.

## 📚 Modules

### ✅ Text Tools (UI Ready)

**Shortcut**: `T` | **Progress**: 12/12

All 12 tools have modern placeholder UIs. Backend implementation pending for 6 tools.

### ✅ PDF Tools (UI Ready)

**Shortcut**: `P` | **Progress**: 12/12

All 12 tools have modern placeholder UIs. Backend implementation pending.

### ✅ Converters (UI Ready)

**Shortcut**: `C` | **Progress**: 12/12 (UI)

All 12 tools have modern placeholder UIs. Backend logic implemented but untested.

### ✅ File & System (Completed)

**Shortcut**: `F` | **Progress**: 12/12 ✅

All 12 tools fully implemented with modern UI and Rust backend.

- ✅ Hash Generator
- ✅ File Renamer
- ✅ Duplicate Finder
- ✅ Disk Usage Analyzer
- ✅ File Splitter
- ✅ Checksum Verifier
- ✅ File Permissions
- ✅ Directory Tree
- ✅ File Watcher
- ✅ Temp Cleaner
- ✅ Metadata Viewer
- ✅ Symlink Manager

### 🚧 Image Tools (Planned)

**Shortcut**: `I` | **Progress**: 0/12

Image Resizer, Compressor, Cropper, Filters, Watermark, Background Remover, Image to Base64, QR Code Generator, Barcode Generator, Metadata Editor, Image Collage, Screenshot Tool

### 🚧 Network (Planned)

**Shortcut**: `N` | **Progress**: 0/12

IP Info, Port Scanner, DNS Lookup, Ping, Traceroute, WHOIS, SSL Checker, HTTP Headers, URL Shortener, Speed Test, MAC Lookup, Subnet Calculator

### 🚧 Quick Commands (Planned)

**Shortcut**: `Q` | **Progress**: 0/12

UUID Generator, Password Generator, Random Number, Lorem Ipsum, Cron Expression, Epoch Converter, JSON Formatter, SQL Formatter, Regex Tester, Color Picker, ASCII Art, Emoji Picker

### 🚧 Dev Tools (Planned)

**Shortcut**: `D` | **Progress**: 0/12

JSON Validator, XML Validator, YAML Validator, HTML Formatter, CSS Formatter, JS Minifier, Git Diff Viewer, API Tester, GraphQL Playground, WebSocket Tester, Snippet Manager, Regex Builder

## ⌨️ Keyboard Shortcuts

### Global Shortcuts

| Shortcut    | Action          |
| ----------- | --------------- |
| `Alt+Space` | Open/Close App  |
| `Ctrl+G`    | Global listener |
| `Esc`       | Close Window    |

### Module Shortcuts

| Key | Module         | Status |
| --- | -------------- | ------ |
| `T` | Text Tools     | ✅     |
| `P` | PDF Tools      | ✅     |
| `C` | Converters     | ⚠️     |
| `F` | File & System  | 🚧     |
| `I` | Image Tools    | 🚧     |
| `N` | Network        | 🚧     |
| `Q` | Quick Commands | 🚧     |
| `D` | Dev Tools      | 🚧     |

See [SHORTCUTS.md](SHORTCUTS.md) for a detailed list of shortcuts.

## 📖 Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Project architecture and technical details
- **[MODULES.md](MODULES.md)** - Detailed planning for all modules and functions
- **[SHORTCUTS.md](SHORTCUTS.md)** - Keyboard shortcuts reference

## 🎯 Project Status

**Total Progress**: 29/144 (20.1%)

| Module         | Status                   | UI    | Backend |
| -------------- | ------------------------ | ----- | ------- |
| File & System  | ⚠️ Redesigned (Untested) | 12/12 | 12/12   |
| Text Tools     | ⚠️ Redesigned (Untested) | 12/12 | 12/12   |
| PDF Tools      | ⚠️ Redesigned (Untested) | 12/12 | 11/12   |
| Converters     | ⚠️ Redesigned (Untested) | 12/12 | 12/12\* |
| Image Tools    | 🚧 Planned               | 0/12  | 0/12    |
| Network        | 🚧 Planned               | 0/12  | 0/12    |
| Quick Commands | 🚧 Planned               | 0/12  | 0/12    |
| Dev Tools      | 🚧 Planned               | 0/12  | 0/12    |

> [!WARNING]
> **Converters**: All 11 converters implemented but untested.

## 🔮 Future Features

- [ ] Implementation of remaining 11 modules
- [ ] Customizable keyboard shortcuts
- [ ] Favorites system
- [ ] History/Recently used
- [ ] Clipboard integration
- [ ] Fuzzy search
- [ ] Command palette
- [ ] Macro recording

## 🤝 Contributing

Contributions are welcome! Please open an issue before submitting a pull request.

## 📄 License

[License information to be added]

---

**Note**: This project is under active development. The Text Tools module is ready; other modules are in the planning/development phase.
