# SimpleTools - Architecture Documentation

## Project Overview

SimpleTools is a modular collection of tools providing quick access via vim-like keyboard shortcuts. It features an Alfred-like interface that opens with `Alt+Space` and provides rapid navigation with specific key combinations for each module.

## Technology Stack

- **Frontend**: Svelte + TypeScript
- **Backend**: Rust (Tauri)
- **UI Framework**: Tauri v2
- **Build Tool**: Vite

## Project Structure

```
SimpleTools/
├── src/                          # Frontend (Svelte)
│   ├── routes/                   # SvelteKit routes
│   │   ├── +page.svelte          # Main page (module selector)
│   │   ├── text/                 # Text Tools module
│   │   ├── pdf/                  # PDF Tools module (in progress)
│   │   ├── convert/              # Converters module (planned)
│   │   ├── file/                 # File & System module (planned)
│   │   ├── image/                # Image Tools module (planned)
│   │   ├── network/              # Network module (planned)
│   │   ├── quickcmd/             # Quick Commands module (planned)
│   │   └── dev/                  # Dev Tools module (planned)
│   ├── themes/                   # Theme files
│   └── app.css                   # Global styles
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── main.rs               # Main Tauri application
│   │   ├── utils.rs              # Window management and positioning
│   │   └── textfunc.rs           # Text Tools functions
│   └── Cargo.toml
└── static/                       # Static files
```

## Module System

### Current Modules (12 Total)

| #   | Module ID | Name          | Shortcut | Status     | Function Count |
| --- | --------- | ------------- | -------- | ---------- | -------------- |
| 1   | text      | Text Tools    | T        | ✅ Ready   | 12             |
| 2   | pdf       | PDF Tools     | P        | ✅ Ready   | 12             |
| 3   | convert   | Converters    | C        | ✅ Ready   | 12             |
| 4   | file      | File & System | F        | 🚧 Planned | 12             |
| 5   | image     | Image Tools   | I        | 🚧 Planned | 12             |
| 6   | network   | Network       | N        | 🚧 Planned | 12             |
| 7   | quickcmd  | Quick Cmds    | Q        | 🚧 Planned | 12             |
| 8   | dev       | Dev Tools     | D        | 🚧 Planned | 12             |
| 9   | TBD       | Coming Soon   | -        | 🚧 Planned | 12             |
| 10  | TBD       | Coming Soon   | -        | 🚧 Planned | 12             |
| 11  | TBD       | Coming Soon   | -        | 🚧 Planned | 12             |
| 12  | TBD       | Coming Soon   | -        | 🚧 Planned | 12             |

**Total**: 144 functions (12 modules × 12 functions)

## Keyboard Shortcuts System

### Global Shortcuts

- **Alt+Space**: Open/Toggle Application (Targeted)
- **Ctrl+G**: Active Development Shortcut (All Platforms)
  - **Windows/macOS**: Native management with `tauri-plugin-global-shortcut`.
  - **Linux**: Background thread listener with `rdev` library (raw input for X11/Wayland compatibility).
- **Esc**: Close current window

### Module Navigation

Access modules from the main screen:

- **T**: Text Tools
- **P**: PDF Tools
- **C**: Converters
- **F**: File & System
- **I**: Image Tools
- **N**: Network
- **Q**: Quick Commands
- **D**: Dev Tools

### Function Navigation (Example: Text Tools)

Inside Text Tools:

- **R**: Regex Tester
- **D**: Text Diff
- **S**: String Tools
- **J**: JWT Decoder
- **L**: Slug Generator
- **T**: Text Strip
- _(6 more functions to be added)_

## Window Layer System

The application uses a 3-layer window system:

### Layer 1: Main Window (main)

- Module selector screen
- Size: 60% width, 65% height of screen
- Position: Centered

### Layer 2: Module Windows (text, pdf, etc.)

- Module function list
- Size: 60% width, 65% height of screen
- Position: Centered

### Layer 3: Function Windows (text/diff, text/regex, etc.)

- Actual tool interface
- Size: Function specific (default 1000×800)
- Position: 35% above center

## Features

### ✅ Completed Features

- [x] Global keyboard shortcut system (Alt+Space)
- [x] Multi-monitor support
- [x] Window positioning based on mouse
- [x] Layered window management
- [x] Text Tools module (12 functions)
- [x] PDF Tools module (12/12 functions)
- [x] Converters module (12/12 functions) - Frontend Ready
- [x] Theme system (5 themes)
- [x] Vim-like navigation

### 🚧 To Do

- [ ] Implementation of remaining 9 modules
- [ ] 12 functions for each module
- [ ] Settings system
- [ ] Keyboard shortcut customization
- [ ] History/favorites system
- [ ] Clipboard integration

## Development Notes

### Adding a New Module

1. Add to `tools` array in `src/routes/+page.svelte`
2. Create `src/routes/{module-id}/` folder
3. Create `src-tauri/src/{module}func.rs` file
4. Register Rust functions in `main.rs`

### Adding a New Function

1. Create `{function-name}/+page.svelte` in the module folder
2. Add Rust backend function to relevant `{module}func.rs`
3. Mark with `#[tauri::command]`
4. Add to `.invoke_handler()` in `main.rs`

## Performance Optimizations

- Window positions are cached
- Last active window is kept in memory
- Modules are loaded lazily on demand
- Fast processing with Rust backend

## Security

- Tauri's built-in security features
- CSP (Content Security Policy) active
- Secure IPC invoke system
- Restricted file system access

## License

[License information to be added]
