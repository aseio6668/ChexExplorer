# DNotepadX - Renaissance Text Editor

A beautiful, feature-rich text editor built with Rust and egui, featuring renaissance-inspired themes and classical elegance.

![Logo](images/chexexplorer.png)

## Features

🎨 **Renaissance-Inspired Themes**
- Classic Renaissance (warm parchment tones)
- Dark Renaissance (rich dark wood)
- Royal Blue (elegant blue palette)
- Forest Green (natural green tones)

📝 **Advanced Text Editing**
- Line numbers with customizable display
- Word wrap toggle
- Find and replace functionality
- Multiple monospace font options
- Customizable font sizes

💾 **File Operations**
- New, Open, Save, Save As
- Save As Copy functionality
- Recent files menu
- Auto-save capabilities

⚙️ **Customization**
- Custom background and text colors
- Font family selection (Consolas, Courier New, etc.)
- Adjustable font sizes
- Theme switching
- Persistent settings

## System Requirements

- Windows, macOS, or Linux
- Rust 1.70+ for building from source

## Installation

### Option 1: Build from Source

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd DNotepadX
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   cargo run --release
   ```

### Option 2: Quick Start in Current Directory

If you're already in a Rust project directory:

```bash
cargo run
```

## Usage

### Basic Operations
- **New File**: `File > New` or Ctrl+N
- **Open File**: `File > Open` or Ctrl+O
- **Save**: `File > Save` or Ctrl+S
- **Save As**: `File > Save As` or Ctrl+Shift+S

### Customization
1. Open `View > Settings` to access the settings panel
2. Choose from preset themes in `View > Themes`
3. Customize colors, fonts, and editor options
4. Settings are automatically saved and restored

### Find & Replace
1. Open `Edit > Find & Replace` or Ctrl+F
2. Enter search and replacement text
3. Use "Find Next", "Replace", or "Replace All"

## Configuration

Settings are automatically saved to:
- **Windows**: `%APPDATA%\dnotepadx\settings.json`
- **macOS**: `~/Library/Application Support/dnotepadx/settings.json`
- **Linux**: `~/.config/dnotepadx/settings.json`

## Renaissance Theme Philosophy

DNotepadX embraces the elegance and sophistication of the Renaissance period through:

- **Warm Color Palettes**: Inspired by aged parchment, rich woods, and gold illumination
- **Classical Typography**: Emphasis on readable monospace fonts
- **Elegant Interface**: Clean lines with subtle ornamental touches
- **Timeless Design**: Balancing beauty with functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - an immediate mode GUI library
- Inspired by classical text editors and renaissance aesthetics
- Font rendering powered by Rust's excellent ecosystem

