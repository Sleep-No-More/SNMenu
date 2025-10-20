# CPMenu - Circular Power Menu for Hyprland

A modern circular power menu for Wayland desktop environments. Built in Rust with GTK3 and Cairo.

Inspired by [wlogout](https://github.com/ArtsyMacaw/wlogout).

## Features

- Circular Layout - Semi-transparent donut ring design
- Customizable Buttons - JSON configuration for actions
- Smooth Animations - Fast, responsive interface
- Keyboard Shortcuts - Quick access with configurable key bindings
- Themeable - GTK CSS styling support
- Wayland Native - Built for modern Wayland compositors

## Screenshots

![CPMenu Circular Power Menu](screenshots/screenshot_20251020_105201.png)

A circular menu layout with wedge-shaped buttons arranged in a ring pattern.

## Installation

### From AUR

```bash
paru -S cpmenu
```

Then use the custom compiled binary:

```bash
sudo cp target/release/cpmenu /usr/local/bin/
```

### Build from Source

Clone this repository and build:

```bash
git clone https://github.com/Sleep-No-More/cpmenu.git
cd cpmenu
cargo build --release
sudo cp target/release/cpmenu /usr/local/bin/
```

### Dependencies

- Rust 1.70 or later
- GTK3 development libraries (libgtk-3-dev)
- Cairo development libraries (libcairo2-dev)

## Configuration

### Layout File

Location: ~/.config/cpmenu/layout

JSON array format:

```json
[
    {
        "label": "lock",
        "action": "hyprlock",
        "text": "Lock",
        "keybind": "l"
    },
    {
        "label": "shutdown",
        "action": "systemctl poweroff",
        "text": "Shutdown",
        "keybind": "s"
    }
]
```

Button properties:

- label: Unique identifier for button styling
- action: Command to execute
- text: Display text on button
- keybind: Optional keyboard shortcut character

### Style File

Location: ~/.config/cpmenu/style.css

Customize button appearance using GTK CSS.

## Hyprland Integration

### Waybar Integration

Add a custom widget to ~/.config/waybar/config.jsonc:

```jsonc
"custom/menu": {
    "format": "Menu",
    "on-click": "cpmenu -l ~/.config/cpmenu/layout -C ~/.config/cpmenu/style.css"
}
```

## Usage

Menu controls:

- Keyboard - Use configured key bindings
- Mouse - Click buttons or press Esc to close
- Hover - Visual feedback on button hover

## Color Customization

Modify button appearance by editing the stylesheet in ~/.config/cpmenu/style.css or modifying src/main.rs for runtime color values.

Rebuild after changes:

```bash
cargo build --release
sudo cp target/release/cpmenu /usr/local/bin/
```

## Project Structure

- src/main.rs - Main application with UI and color logic
- src/circular_layout.rs - Circular button layout rendering
- src/config.rs - Configuration loading and color parsing
- layout - Default button configuration (JSON array)
- style.css - GTK CSS styling
- Cargo.toml - Rust dependencies and project metadata

## Performance

- Smooth 60 FPS rendering with Cairo
- Low CPU usage
- Fast startup time
- Minimal memory footprint

## Troubleshooting

### Configuration not loading

Verify layout and style files exist at configured paths.

### CSS styling not applied

Ensure style.css syntax is valid GTK CSS.

## Development

### Building for Development

```bash
cargo build
./target/debug/cpmenu
```

### Building Release

```bash
cargo build --release
./target/release/cpmenu
```

### Testing Custom Changes

```bash
# Test with specific layout
./target/debug/cpmenu -l ~/.config/cpmenu/layout -C ~/.config/cpmenu/style.css

# With debug logging
RUST_LOG=debug ./target/debug/cpmenu
```

## License

CPMenu is released under the MIT License. See LICENSE file for details.
