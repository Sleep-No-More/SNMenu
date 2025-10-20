# CPMenu - Circular Power Menu for Hyprland

A modern circular power menu for Wayland desktop environments. Built in Rust with GTK3 and Cairo.

## Current Setup: v0.1.0 (Custom Red Shutdown Button)

October 19, 2025 - Enhanced Rust implementation with custom red shutdown button

### Features

- Red Shutdown Button - Highlighted in bright red for visual distinction
- Light Blue Base Buttons - Clean professional color scheme
- 60 FPS Smooth Animations - Icon scaling on hover
- 6 Power Actions - Lock, Hibernate, Logout, Shutdown, Suspend, Reboot
- Professional Design - Semi-transparent donut ring layout
- Keyboard Shortcuts - Quick access with letter keys (L, H, E, S, U, R)
- Wayland Optimized - Full integration with Hyprland
- JSON Configuration - Customizable buttons and actions

## Screenshots

![CPMenu Circular Power Menu](screenshots/cpmenu-menu.png)

The circular power menu displays 6 wedge-shaped buttons arranged in a donut pattern:

- Lock (top-left)
- Hibernate (left)
- Logout (bottom-left)
- Shutdown (bottom, highlighted in red)
- Suspend (right)
- Reboot (top-right)

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

- label: Unique identifier (buttons with "shutdown" or "power" are colored red)
- action: Command to execute
- text: Display text on button
- keybind: Optional keyboard shortcut character

### Style File

Location: ~/.config/cpmenu/style.css

Customize button appearance using GTK CSS.

## Hyprland Integration

### Keybinding

Add to ~/.config/hypr/keybindings.conf:

```bash
bind = $mainMod SHIFT, P, exec, cpmenu -l ~/.config/cpmenu/layout -C ~/.config/cpmenu/style.css
```

### Waybar Integration

Add to ~/.config/waybar/config.jsonc:

```jsonc
"custom/power": {
    "format": "󰐥",
    "tooltip": false,
    "on-click": "cpmenu -l ~/.config/cpmenu/layout -C ~/.config/cpmenu/style.css"
}
```

## Usage

### Launch Methods

1. Keyboard: Super + Shift + P
2. Waybar: Click power button
3. Terminal: cpmenu

### Menu Controls

When menu is open:

- L - Lock screen
- H - Hibernate
- E - Logout
- S - Shutdown (red button)
- U - Suspend
- R - Reboot
- Esc - Close menu
- Mouse - Click any button

## Color Customization

To modify button colors, edit src/main.rs:

```rust
let (base_color, hover_color) = if btn.label.contains("shutdown") {
    // Shutdown button - red
    (
        parse_color_with_alpha("#DC5050", 0.8),
        parse_color_with_alpha("#E07070", 0.9),
    )
} else {
    // Other buttons - light blue
    (
        parse_color_with_alpha("#81A1C1", 0.35),
        parse_color_with_alpha("#5E81AC", 0.55),
    )
};
```

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

- 60 FPS smooth rendering with Cairo
- Minimal CPU usage during idle
- Fast startup time (approximately 100ms)
- Low memory footprint (approximately 10MB)

## Troubleshooting

### Icon not found warnings

Ensure cpmenu is installed system-wide and icons are available at:

- /usr/share/cpmenu/assets/ (AUR package)
- /usr/local/share/cpmenu/assets/ (manual installation)

### JSON parsing errors

Verify layout file is valid JSON array format, not newline-delimited JSON.

### CSS styling not applied

Check style.css syntax - GTK CSS has different rules than web CSS.

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

## Version History

- v0.1.0 (October 19, 2025) - Custom compiled with red shutdown button enhancement
- v0.0.9 - Previous C implementation
- v0.0.1 - Initial release

## License

CPMenu is released under the MIT License. See LICENSE file for details.
