# Advanced Keyboard Configuration with Kanata

> *Transform any keyboard into a programmable powerhouse*

This directory contains an optional **Kanata** configuration that brings QMK-level keyboard customization to your regular keyboard. This is completely independent from HyprGrid and provides system-wide keyboard enhancements.

## What is Kanata?

**Kanata** is a powerful keyboard remapper for Linux that works at the kernel level. It enables:

- **Home Row Mods** - Dual-function keys: tap for letters, hold for modifiers
- **Compose Key Support** - System-wide compose sequences for accents
- **Multiple Layers** - Just like QMK firmware on custom keyboards
- **System-Wide** - Works in TTY, X11, Wayland (including Hyprland)
- **No GUI Required** - Operates at kernel level via evdev/uinput

## Why Use This Config?

The provided configuration implements:

1. **Modified GACS Home Row Mods Layout** (optimized for Hyprland/Linux)
   - `A` = LeftAlt (tap `a`, hold for Left Alt)
   - `S` = Shift (tap `s`, hold for Shift)
   - `D` = Ctrl (tap `d`, hold for Ctrl)
   - `F` = Super/Meta (tap `f`, hold for Super)
   - Mirrored on right hand: `J/K/L/;/.`
   - `;` = LeftAlt (tap `;`, hold for Left Alt)
   - `.` = RightAlt (tap `.`, hold for Right Alt)

2. **Two Configurations Available**
   - **kanata.kbd**: Simplified 2-layer config (base ↔ nav) without Portuguese accents
   - **kanata.kbd.example**: Full 3-layer config with Portuguese accents layer

   **Simplified Config (kanata.kbd):**
   - **Base Layer**: Normal typing with home row mods
   - **Navigation Layer**: Vim-style arrow keys (h/j/k/l → ←/↓/↑/→)
   - Tap physical Super key to toggle: base ↔ nav

   **Full Config (kanata.kbd.example):**
   - **Base Layer**: Normal typing with home row mods
   - **Navigation Layer**: Vim-style arrow keys (h/j/k/l → ←/↓/↑/→)
   - **Accents Layer**: Direct Portuguese accented characters
   - Tap physical Super key to cycle: base → nav → accents → base

3. **Portuguese Accents Layer** (kanata.kbd.example only - works in terminals!)
   - Direct character access via layer switching
   - Vowel E: `e`→é, `r`→ê
   - Vowel I: `i`→í
   - Vowel U: `u`→ú
   - Vowel O: `o`→ó, `p`→õ, `[`→ô
   - Vowel A: `a`→á, `s`→ã, `d`→â, `f`→à
   - Cedilla: `c`→ç
   - Hold `L` (Shift) for uppercase: á→Á, ç→Ç, etc.

4. **Extra Ergonomics**
   - Caps Lock → Tap for Escape, Hold to toggle Caps Lock on/off

## Installation on Arch Linux

### 1. Install Kanata

```bash
# Install from AUR (choose one)
yay -S kanata-bin
# Verify installation
kanata --version
```

### 2. Choose and Configure Your Config

**Two configurations available:**
- **kanata.kbd**: Simplified config with 2 layers (base ↔ nav) - **recommended for most users**
- **kanata.kbd.example**: Full config with 3 layers including Portuguese accents

```bash
# Create system-wide config directory
sudo mkdir -p /etc/kanata

# Option A: Use simplified config (recommended)
sudo cp kanata.kbd /etc/kanata/kanata.kbd

# Option B: Use full config with Portuguese accents
sudo cp kanata.kbd.example /etc/kanata/kanata.kbd

# IMPORTANT: Find your keyboard device path
ls -l /dev/input/by-id/ | grep kbd

# Example output:
# usb-SONiX_Satechi_Compact_Keyboard-event-kbd -> ../event9
# usb-Logitech_USB_Keyboard-event-kbd -> ../event5

# Edit the config and update the linux-dev line with YOUR keyboard
sudo nvim /etc/kanata/kanata.kbd
```

**⚠️ CRITICAL:** You MUST update the `linux-dev` line in the config file with your actual keyboard device path, or Kanata won't be able to intercept your keyboard!

### 3. Set Up System Service

Create a system-wide service to run Kanata as root automatically at boot:

```bash
# Create the system service file
sudo tee /etc/systemd/system/kanata.service > /dev/null <<'EOF'
[Unit]
Description=Kanata keyboard remapper
Documentation=https://github.com/jtroo/kanata
After=local-fs.target

[Service]
Type=simple
ExecStart=/usr/bin/kanata --cfg /etc/kanata/kanata.kbd
Restart=on-failure
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd to recognize the new service
sudo systemctl daemon-reload

# Enable and start the service
sudo systemctl enable kanata.service
sudo systemctl start kanata.service

# Check status
sudo systemctl status kanata.service
```

## Testing the Configuration

### Find Your Keyboard Device

```bash
# List input devices by path (persistent names)
ls -l /dev/input/by-path/ | grep kbd

# List input devices by ID (most reliable)
ls -l /dev/input/by-id/ | grep kbd

# View all event devices
ls -l /dev/input/event*
```

### Test Manually First

Before enabling the service, test the config manually:

```bash
# Run in foreground to see any errors (requires root for device access)
sudo kanata --cfg /etc/kanata/kanata.kbd

# In another terminal, test typing
# - Single tap 'a' should type 'a'
# - Hold 'a' + tap ' + release + tap 'e' should type 'é'
# - Hold 'd' + tap 'c' should trigger Ctrl+C
```

Press `Ctrl+C` to stop the manual test.

**Note:** Running as root provides full access to input devices without needing udev rules

### Validate Configuration

Before running, you can check your config file for syntax errors:

```bash
# Validate the configuration
sudo kanata --cfg /etc/kanata/kanata.kbd --check

# If valid, you'll see no errors and the command exits successfully
```

## Usage Guide

### Home Row Mods

**Left Hand:**
- `A` = LeftAlt modifier
- `S` = Shift modifier
- `D` = Ctrl modifier
- `F` = Super/Meta modifier

**Right Hand (mirrored):**
- `J` = Super/Meta modifier
- `K` = Ctrl modifier
- `L` = Shift modifier
- `;` = LeftAlt modifier
- `.` = RightAlt modifier

**Examples:**
- `Super+T` → Hold `F` + tap `T`
- `Ctrl+C` → Hold `D` + tap `C`
- `Shift+A` → Hold `S` + tap `A` (types capital 'A')

### Portuguese Accents (kanata.kbd.example only)

The full config (`kanata.kbd.example`) includes a dedicated accents layer for typing Portuguese characters directly:

1. Tap physical **Super key twice** (from base) to enter accents layer
2. Vowel keys become accented versions:
   - `e`→é, `r`→ê
   - `i`→í, `u`→ú
   - `o`→ó, `p`→õ, `[`→ô
   - `a`→á, `s`→ã, `d`→â, `f`→à
   - `c`→ç
3. Hold `L` (Shift) for uppercase: á→Á, ç→Ç
4. Tap Super again to return to base layer

**Example:** Type "não": (base layer) `n` → (tap Super twice) `s` `o` → (tap Super) done!

**Works everywhere** including terminals, TTY, and GUI apps!

### Caps Lock Behavior

- **Tap** Caps Lock → `Escape`
- **Hold** Caps Lock → Toggle Caps Lock on/off

Perfect for Vim users! Quick tap gives you Escape, and when you need to TYPE IN ALL CAPS, just hold the key to toggle Caps Lock.

## Customization

### Adjusting Timeouts

Edit `/etc/kanata/kanata.kbd` (with sudo) and modify the timing values:

```lisp
(defvar
  tap-time 150    ;; How fast to resolve as tap (ms)
  hold-time 200   ;; How long to hold for modifier (ms)
)
```

- **Slower typing?** Increase both values (try 180/250)
- **Faster typing?** Decrease both values (try 130/180)
- **Too many accidental modifiers?** Increase `hold-time` only

### Adding More Layers

You can extend the config with navigation, symbol, or number pad layers. See Kanata's [official examples](https://github.com/jtroo/kanata/tree/main/cfg_samples).

### Device-Specific Config

If you have multiple keyboards, modify the `linux-dev` line:

```lisp
(defcfg
  linux-dev /dev/input/by-path/YOUR-KEYBOARD-PATH
)
```

Or use `/dev/input/by-id/` for persistent device identification.

## Troubleshooting

### Kanata won't start

```bash
# Check service logs
sudo journalctl -u kanata.service -f

# Common issues:
# - Wrong device path → Update linux-dev in config
# - Config syntax error → Test manually to see error messages
```

**Error: "No keyboard devices were found" or "No such file or directory"**

This means the device path in your config doesn't match your actual keyboard:

```bash
# 1. Find your keyboard device
ls -l /dev/input/by-id/ | grep kbd

# 2. Edit your config
sudo vim /etc/kanata/kanata.kbd

# 3. Update the linux-dev line with the full path, e.g.:
linux-dev /dev/input/by-id/usb-Your_Keyboard_Name-event-kbd

# 4. Validate and test
sudo kanata --cfg /etc/kanata/kanata.kbd --check
sudo kanata --cfg /etc/kanata/kanata.kbd --debug
```

### Home row mods trigger accidentally

This is normal when first learning! Solutions:

1. **Increase hold timeout:** Change `hold-time` from 200 to 250ms
2. **Practice typing faster:** The faster you type normally, the better it works
3. **Adjust per-finger timings:** Pinkies are slower, can have longer timeouts

### Portuguese accents not working

**For kanata.kbd.example users:**

If the accents layer isn't producing accented characters:

1. **Verify you're in the accents layer:**
   - Tap physical Super key twice from base layer
   - You should now be able to type accented characters directly

2. **Check for Unicode support:**
   - Most modern terminals and applications support Unicode by default
   - If characters appear as boxes, your terminal may need UTF-8 locale:
     ```bash
     # Check current locale
     locale
     # Should show UTF-8 entries like en_US.UTF-8 or pt_BR.UTF-8
     ```

3. **Debug mode:**
   ```bash
   # Run Kanata in debug mode to see layer switches
   sudo kanata --cfg /etc/kanata/kanata.kbd --debug
   # Tap Super twice and watch for layer change to 'accents'
   ```

### Conflicts with other remapping tools

Disable any other keyboard remapping tools:

```bash
# If you have keyd running
sudo systemctl stop keyd
sudo systemctl disable keyd

# If you have xremap or similar
# Stop those services as well
```

Only one keyboard remapper should run at a time.

### RightAlt key behavior

The `.` (period) key sends RightAlt when held. This is useful for:

- **System-wide compose key sequences** (if you have compose configured in your Linux setup)
- **Application-specific RightAlt shortcuts**
- **International keyboard layouts** that use RightAlt for special characters

**Note:** This config doesn't set up compose key functionality. If you want to use the `.` key for compose sequences, configure it through your Linux system settings or desktop environment.

## Useful Commands

```bash
# Validate config file for syntax errors
sudo kanata --cfg /etc/kanata/kanata.kbd --check

# Run Kanata in debug mode (see what it's doing)
sudo kanata --cfg /etc/kanata/kanata.kbd --debug

# Run Kanata with trace logging (very verbose)
sudo kanata --cfg /etc/kanata/kanata.kbd --trace

# Reload config after changes (restart service)
sudo systemctl restart kanata.service

# View logs in real-time
sudo journalctl -u kanata.service -f

# Check if Kanata is running
sudo systemctl status kanata.service

# Stop Kanata temporarily
sudo systemctl stop kanata.service

# Start Kanata
sudo systemctl start kanata.service

# Disable auto-start
sudo systemctl disable kanata.service
```

## Learning Curve

**Week 1:** Expect some frustration. Your typing speed will drop as you adjust to home row mods.

**Week 2:** Muscle memory starts forming. Accidental modifier triggers decrease.

**Week 3+:** Home row mods become natural. You'll wonder how you ever used dedicated modifier keys.

**Tip:** Practice with typing tests (monkeytype.com) while adjusting to the new layout.

## Resources

- **Kanata GitHub:** https://github.com/jtroo/kanata
- **Configuration Guide:** https://github.com/jtroo/kanata/wiki/Configuration-guide
- **Home Row Mods Guide:** https://precondition.github.io/home-row-mods
- **Community Configs:** https://github.com/jtroo/kanata/tree/main/cfg_samples

## Philosophy

This configuration follows the **"Light Cycle Paradigm"** - just like HyprGrid, it's about efficiency and eliminating unnecessary movement. By bringing modifiers to your home row and adding Portuguese accents via tap-dance, you minimize hand movement and maximize productivity.

Your fingers stay on the home row. Everything is within reach. The keyboard becomes an extension of thought.

---

*End of line.*
