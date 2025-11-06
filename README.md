# HyprGrid

> *"Greetings, Program. Welcome to the Grid."*

HyprGrid is a keyboard-driven mouse controller for Hyprland, inspired by the sleek efficiency of TRON's digital world. Control your cursor with the precision of a light cycle – no mouse required.

## What is HyprGrid?

HyprGrid transforms your screen into a grid of labeled cells. Type two letters to select any point on your screen, then click with a keystroke. It's fast, keyboard-driven, and materializes only when you need it.

Think of it as summoning a light cycle: press your hotkey, the grid appears, you make your selection, execute your action, and it de-rezzes instantly. No background processes, no overhead – just pure, efficient navigation.

## Features

- **Lightning Fast:** Appears instantly, executes a single action, then terminates
- **Keyboard-Driven:** Navigate your entire screen without touching the mouse
- **Multi-Monitor Support:** Works seamlessly across multiple displays
- **Hyprland Native:** Designed specifically for the Hyprland compositor
- **Lightweight:** No persistent background process – runs only when summoned

## Prerequisites

Before entering the Grid, ensure you have:

- **Hyprland** compositor (obviously)
- **ydotoold** running in the background for input synthesis
  ```bash
  # Install ydotoold (Arch Linux example)
  sudo pacman -S ydotool

  # Enable and start the service
  sudo systemctl enable --now ydotool
  ```

## Installation

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/HyprGrid.git
cd HyprGrid

# Build the project
cargo build --release

# Copy the binary to your Hyprland config directory
cp target/release/hyprgrid ~/.config/hypr/
```

## Configuration

### 1. Create the Grid Configuration

Create `~/.config/hypr/hg_config.conf` to define your grid layout and monitor setup:

```toml
# hg_config.conf

# Define the grid dimensions (rows and columns)
grid_rows = 10
grid_cols = 20

# Define each monitor available to HyprGrid
# The name must match the output of `hyprctl monitors`
[[monitors]]
name = "DP-1"
width = 2560
height = 1440

[[monitors]]
name = "HDMI-A-1"
width = 1920
height = 1080
```

### 2. Configure Hyprland

Add these lines to your `~/.config/hypr/hyprland.conf`:

```hyprlang
# Keybind to summon HyprGrid (SUPER + S in this example)
bind = $mainMod, S, exec, ~/.config/hypr/hyprgrid

# Window rules to make HyprGrid a proper overlay
windowrulev2 = float, initialTitle:^(HyprGrid)$
windowrulev2 = fullscreen, initialTitle:^(HyprGrid)$
windowrulev2 = noborder, initialTitle:^(HyprGrid)$
windowrulev2 = noshadow, initialTitle:^(HyprGrid)$
windowrulev2 = opacity 0.8, initialTitle:^(HyprGrid)$
```

Reload your Hyprland configuration or restart Hyprland.

## Usage

1. **Summon the Grid:** Press your configured hotkey (e.g., `SUPER + S`)
2. **Select Your Target:** The grid appears with letter-pair labels. Type the two letters corresponding to your target cell (e.g., `aj`)
3. **Execute Action:**
   - Press `SPACE` for a **left click**
   - Press `ENTER` for a **right click**
4. **De-rezz:** The grid vanishes, your action is complete

It's that simple. The program materializes, performs its function, and terminates – just like a light cycle completing its circuit.

## How It Works

HyprGrid operates on the "Light Cycle Paradigm" – it's not a persistent background process. Each invocation:

1. Queries Hyprland for your active monitor
2. Loads your grid configuration
3. Renders the labeled grid overlay
4. Captures your input (2 letters + action key)
5. Sends the appropriate `ydotool` commands
6. Terminates immediately

No daemons, no servers, no persistent state. Just instant execution.

## Troubleshooting

### The grid doesn't appear
- Verify `hyprgrid` binary is in the correct path
- Check that your Hyprland window rules are applied
- Run the binary manually from terminal to see any error messages

### Mouse doesn't click
- Ensure `ydotoold` service is running: `systemctl status ydotool`
- Verify your user has permission to use ydotool
- Check that ydotool socket is accessible

### Wrong monitor resolution
- Run `hyprctl monitors` to see your monitor names and resolutions
- Update your `hg_config.conf` with the correct values
- Ensure monitor names match exactly

## Contributing

The Grid is open source. If you have improvements, bug fixes, or new features to propose, feel free to open an issue or submit a pull request.

## License

[Add your license here]

## Acknowledgments

Inspired by various grid-based mouse control tools and the aesthetic efficiency of TRON's digital world.

---

*End of line.*
