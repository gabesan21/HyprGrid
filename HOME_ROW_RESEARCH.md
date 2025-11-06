<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

## Home Row Mods on Regular Keyboards in Arch Linux

Home row mods (homerow modifiers) let you place modifier keys like Shift, Control, Alt, and Super/Win on your home row keys (ASDF and JKL;) using a dual-function "tap-hold" behavior. When you **tap** a key, it types the letter; when you **hold** it, it acts as a modifier.[^1_1][^1_2][^1_3]

Since you're using Arch Linux, you have several software solutions to implement home row mods on any keyboard, even non-programmable ones.

## Best Solution for Arch: Kanata

**Kanata** is the most recommended option for Arch Linux users. It's a powerful keyboard remapper written in Rust that works at the kernel level, making it compatible with both X11 and Wayland (including Hyprland, which you use).[^1_4][^1_5][^1_6]

### Installing Kanata on Arch

You can install Kanata from the AUR using your preferred AUR helper:

```bash
yay -S kanata
# or for the binary version
yay -S kanata-bin
```

Alternatively, install manually from AUR:[^1_7][^1_8]

```bash
git clone https://aur.archlinux.org/kanata.git
cd kanata
makepkg -si
```


### Basic Kanata Configuration

Create a configuration file at `~/.config/kanata/config.kbd`:[^1_9][^1_10][^1_1]

```lisp
(defcfg
  process-unmapped-keys yes
  linux-dev /dev/input/by-path/platform-i8042-serio-0-event-kbd
)

(defsrc
  caps a s d f j k l ;
)

(defvar
  tap-time 150
  hold-time 200
)

(defalias
  ;; Home row mods - left hand
  a (tap-hold $tap-time $hold-time a lmet)  ;; Super/Win
  s (tap-hold $tap-time $hold-time s lalt)  ;; Alt
  d (tap-hold $tap-time $hold-time d lsft)  ;; Shift
  f (tap-hold $tap-time $hold-time f lctl)  ;; Control
  
  ;; Home row mods - right hand
  j (tap-hold $tap-time $hold-time j rctl)  ;; Control
  k (tap-hold $tap-time $hold-time k rsft)  ;; Shift
  l (tap-hold $tap-time $hold-time l ralt)  ;; Alt
  ; (tap-hold $tap-time $hold-time ; rmet)  ;; Super/Win
  
  ;; Caps Lock as Escape when tapped, Control when held
  escctrl (tap-hold 100 100 esc lctl)
)

(deflayer base
  @escctrl @a @s @d @f @j @k @l @;
)
```


### Running Kanata

Kanata needs to access `/dev/input` devices, so it requires root privileges:[^1_11][^1_4]

```bash
sudo kanata -c ~/.config/kanata/config.kbd
```

To avoid using sudo, follow the [udev rules setup](https://github.com/jtroo/kanata/wiki/Avoid-using-sudo-on-Linux).[^1_4]

### Setting Up as a Systemd Service

Create a systemd service to run Kanata automatically:[^1_12][^1_11]

```bash
sudo nano /etc/systemd/system/kanata.service
```

Add this content:

```ini
[Unit]
Description=Kanata keyboard remapper
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/kanata -c /home/YOUR_USERNAME/.config/kanata/config.kbd
Restart=always
User=root
Group=root

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl enable kanata.service
sudo systemctl start kanata.service
```


## Optimizing Timings to Prevent Misfires

The biggest challenge with home row mods is avoiding **false activations** (misfires) when typing fast. Here are key timing adjustments:[^1_2][^1_13][^1_14]

### Individual Finger Timings

Different fingers have different speeds. Customize timings per key:[^1_15][^1_16]

- **Shift (index fingers)**: 150-160ms hold time
- **Control (middle fingers)**: 250-300ms hold time
- **Alt (ring fingers)**: 350-400ms hold time
- **Super/Win (pinkies)**: 400-450ms hold time


### Advanced: Typing Layer Method

For fast typists, use a "typing layer" that disables home row mods during rapid typing:[^1_16][^1_15]

```lisp
(defvirtualkeys
  to-base (layer-switch base)
)

(defalias
  .tp (multi
        (layer-switch typing)
        (on-idle 55 tap-vkey to-base))
)

(deflayermap (base)
  f (tap-hold 0 160 (multi f @.tp) lsft)
  d (tap-hold 0 250 (multi d @.tp) lctl)
  ;; Add similar for other keys
)

(deflayermap (typing)
  ;; Plain letters, no modifiers
  a a b b c c d d e e f f
  ;; etc...
)
```

This automatically switches to a layer without modifiers when you type quickly, eliminating most misfires.[^1_15][^1_16]

## Alternative: KMonad

KMonad is another option, though Kanata is generally preferred for Arch:[^1_1][^1_11]

```bash
yay -S kmonad-bin
# or
yay -S kmonad-git
```

KMonad uses a similar configuration syntax but has fewer features than Kanata.[^1_17][^1_5][^1_4]

## Quick Tips for Success

1. **Start with longer timeouts** (250-300ms) and gradually decrease them as you adapt[^1_16][^1_15]
2. **Use bilateral combinations**: Only activate modifiers when pressing keys on the opposite hand[^1_2][^1_1]
3. **Fix problematic bigrams** using `tap-hold-release-keys` for common letter combinations like "as" or "df"[^1_18][^1_15]
4. **Test your typing speed** on sites like monkeytype.com while adjusting timings[^1_16]
5. **Consider putting Shift on thumb keys** instead of home row to avoid the most common misfires[^1_14][^1_19]

The learning curve typically takes 1-2 weeks to feel comfortable, but the ergonomic benefits and reduced finger travel make it worthwhile for many users.[^1_20][^1_21]
<span style="display:none">[^1_22][^1_23][^1_24][^1_25][^1_26][^1_27][^1_28][^1_29][^1_30][^1_31][^1_32][^1_33][^1_34][^1_35][^1_36][^1_37][^1_38][^1_39][^1_40][^1_41][^1_42][^1_43][^1_44][^1_45][^1_46][^1_47][^1_48][^1_49][^1_50][^1_51][^1_52][^1_53][^1_54][^1_55][^1_56][^1_57][^1_58]</span>

<div align="center">⁂</div>

[^1_1]: https://precondition.github.io/home-row-mods

[^1_2]: https://www.geewiz.dev/notes/Knowledge+Base/Home+Row+Mods

[^1_3]: https://dygma.com/blogs/ergonomics/home-row-modifiers-vs-thumbkeys

[^1_4]: https://docs.rs/crate/kanata/1.0.8

[^1_5]: https://www.reddit.com/r/KeyboardLayouts/comments/ualj0l/kanata_a_software_keyboard_remapper_for_windows/

[^1_6]: https://github.com/jtroo/kanata

[^1_7]: https://aur.archlinux.org/packages/kanata

[^1_8]: https://aur.archlinux.org/packages/kanata-bin

[^1_9]: https://white8785.com/home row mods

[^1_10]: https://github.com/dreamsofcode-io/home-row-mods

[^1_11]: https://witnessjo.github.io/posts/linux/kmonad/

[^1_12]: https://www.swe-devops.com/posts/kmonad-service-systemd/

[^1_13]: https://www.reddit.com/r/ErgoMechKeyboards/comments/1f55kky/home_row_mods/

[^1_14]: https://www.reddit.com/r/ErgoMechKeyboards/comments/16d5lep/a_guide_to_home_row_mods/

[^1_15]: https://github.com/jtroo/kanata/discussions/1656

[^1_16]: https://github.com/jtroo/kanata/discussions/1455

[^1_17]: https://www.reddit.com/r/rust/comments/w0zqk4/kanata_v105_an_advanced_keyboard_remapper_for/

[^1_18]: https://github.com/jtroo/kanata/blob/main/cfg_samples/home-row-mod-advanced.kbd

[^1_19]: https://forum.ultimatehackingkeyboard.com/t/miryoku-and-homerow-mods/723

[^1_20]: https://www.reddit.com/r/ErgoMechKeyboards/comments/1du4mpc/homerow_mod_users_will_the_discomfort_be_over/

[^1_21]: https://rho.org.uk/2022/03/no_place_like_home_row.html

[^1_22]: https://www.youtube.com/watch?v=oaAuhQm9RzM

[^1_23]: https://www.youtube.com/watch?v=PBjV6nxQpqs

[^1_24]: https://www.youtube.com/watch?v=MGs6dBn9irg

[^1_25]: https://micro.thomasbaart.nl/2024/06/13/a-guide-to-home-row-mods-by-precondition/

[^1_26]: https://www.youtube.com/watch?v=2K9vo7SfFy0

[^1_27]: https://github.com/precondition/precondition.github.io/discussions/26

[^1_28]: https://www.reddit.com/r/ErgoMechKeyboards/comments/1f18d8h/i_have_fixed_home_row_mods_in_qmk_for_everyone/

[^1_29]: https://siddhantkhisty.com/a-painless-journey-into-arch-linux

[^1_30]: https://havn.blog/2024/03/03/a-good-way.html

[^1_31]: https://github.com/bsag/qmk_custom

[^1_32]: https://www.reddit.com/r/termux/comments/rjgnp2/help_install_kmonad_package_download_binary_arch/

[^1_33]: https://github.com/xremap/xremap?tab=readme-ov-file

[^1_34]: https://github.com/kmonad/kmonad/blob/master/doc/installation.md?plain=1

[^1_35]: https://linuxconfig.org/reprogram-keyboard-keys-with-xmodmap

[^1_36]: https://www.youtube.com/watch?v=g_WUusDluLw

[^1_37]: https://dev.to/shanu-kumawat/boost-your-linux-productivity-remapping-useless-keys-with-kanata-3ih5

[^1_38]: https://github.com/kmonad/kmonad/issues/283

[^1_39]: https://valleyoflostcode.blogspot.com/2020/01/how-to-remap-keys-in-linux.html

[^1_40]: https://mcanueste.com/posts/kanata-to-rule-them-all/

[^1_41]: https://github.com/xremap/xremap

[^1_42]: https://archlinux.org/packages/extra/x86_64/kmonad/

[^1_43]: https://bbs.archlinux.org/viewtopic.php?id=293883

[^1_44]: https://shom.dev/start/using-kanata-to-remap-any-keyboard/

[^1_45]: https://aur.archlinux.org/packages/kmonad-git

[^1_46]: https://linuxtldr.com/arch-aur-manual-install-guide/

[^1_47]: https://www.youtube.com/watch?v=jvJ3f4HHiMY

[^1_48]: https://dev.to/nabbisen/installing-aur-packages-bdf

[^1_49]: https://blog.getreu.net/20250826-muscle-memory-friendly-home-row-mods/

[^1_50]: https://linuxgenie.net/how-to-install-aur-packages-on-arch-linux/

[^1_51]: https://forum.ultimatehackingkeyboard.com/t/home-row-mods-or-other-good-solutions-for-ctrl-c-and-the-like/52

[^1_52]: https://github.com/SharafatKarim/chaotic-AUR-installer

[^1_53]: https://www.reddit.com/r/ErgoMechKeyboards/comments/om99cn/please_give_me_your_tips_for_home_row_mods_with/

[^1_54]: https://github.com/jtroo/kanata/discussions/518

[^1_55]: https://www.reddit.com/r/ErgoMechKeyboards/comments/x20i2b/home_row_modifiers_and_rollover/

[^1_56]: https://www.reddit.com/r/KeyboardLayouts/comments/1k3s118/i_really_like_my_kanata_layout_but_dont_have/

[^1_57]: https://www.reddit.com/r/archlinux/comments/1cq749i/installing_from_aur/

[^1_58]: https://www.youtube.com/watch?v=Qzo8PtDzzdA


---

## Yes, Home Row Mods Work in TTY/Terminal-Only Mode

**Good news**: Kanata (and similar tools) work perfectly in **TTY/console sessions** without any GUI or display server. This is one of their main advantages over desktop environment-specific solutions.

### How It Works at the Kernel Level

Kanata operates at the **kernel input level** using the Linux **evdev** and **uinput** subsystems. This means it:[^2_1][^2_2][^2_3]

1. **Reads input** from `/dev/input/event*` devices (evdev)[^2_3][^2_1]
2. **Processes** the key events according to your configuration
3. **Injects remapped keys** back through `/dev/uinput` as a virtual keyboard device[^2_2][^2_1][^2_3]

Because this happens at the kernel level, it works **everywhere**:[^2_4][^2_5][^2_6]

- ✅ TTY/virtual consoles (Ctrl+Alt+F1-F6)
- ✅ X11 sessions
- ✅ Wayland compositors (including Hyprland)
- ✅ Login screens
- ✅ Virtual machines running on the system


### The Same Config Works Everywhere

Your home row mods configuration will work identically whether you're:

- In a TTY console typing commands
- In your Hyprland session
- At the login prompt
- Running terminal applications like vim/neovim in TTY

The remapping is **completely independent** of the display server or desktop environment.[^2_7][^2_6][^2_4]

## Alternative: keyd

Another excellent option that explicitly advertises TTY support is **keyd**:[^2_5][^2_6][^2_4]

```bash
yay -S keyd
```

**keyd** is specifically described as working "across display server boundaries (e.g wayland/X/tty)". It's similar to Kanata but uses a slightly different configuration syntax.[^2_6][^2_4]

### Basic keyd Configuration

Create `/etc/keyd/default.conf`:[^2_5][^2_7]

```ini
[ids]
*

[main]
# Home row mods
a = overload(meta, a)
s = overload(alt, s)
d = overload(shift, d)
f = overload(control, f)

j = overload(control, j)
k = overload(shift, k)
l = overload(alt, l)
; = overload(meta, ;)

# Caps as Escape/Control
capslock = overload(control, esc)
```

Enable the service:

```bash
sudo systemctl enable keyd
sudo systemctl start keyd
```

Reload config after changes:

```bash
sudo keyd reload
```


### keyd Emergency Panic Sequence

If you mess up your configuration in TTY and can't type properly, keyd has a built-in panic sequence: **Backspace + Escape + Enter** simultaneously will force keyd to terminate.[^2_4][^2_6]

## TTY-Specific Considerations

### Known Issue with Kanata and TTY Login

There's a **reported issue** with Kanata interfering with TTY password entry if it's started too early. The workaround is to:[^2_8]

1. **Not** run Kanata as a system service that starts before login
2. Start it as a **user service** after you've logged in[^2_8]

Example user systemd service at `~/.config/systemd/user/kanata.service`:

```ini
[Unit]
Description=Kanata keyboard remapper
After=default.target

[Service]
Type=simple
ExecStart=/usr/bin/kanata -c %h/.config/kanata/config.kbd
Restart=always

[Install]
WantedBy=default.target
```

Enable it:

```bash
systemctl --user enable kanata.service
systemctl --user start kanata.service
```


### Why keyd Might Be Better for TTY-Heavy Usage

Given your preference for terminal-only work, **keyd** might be slightly better because:[^2_7][^2_4]

- It's explicitly designed for system-wide operation including TTY[^2_6][^2_5]
- Fewer reported TTY login issues compared to Kanata[^2_8]
- Simpler configuration syntax for basic use cases[^2_5][^2_7]
- Built-in panic sequence for emergencies[^2_4][^2_6]


## Both Tools Are System-Wide

The key takeaway: both **Kanata** and **keyd** work at the kernel level through uinput/evdev, making them completely **display-server agnostic**. Your home row mods will function identically whether you're in:[^2_1][^2_3][^2_4]

- A bare TTY console running `vim`
- Your terminal emulator in Hyprland
- A tmux session
- The bootup process

This kernel-level approach is exactly what makes these tools so powerful for Arch users who frequently switch between GUI and terminal-only environments.[^2_9][^2_3][^2_4]
<span style="display:none">[^2_10][^2_11][^2_12][^2_13][^2_14][^2_15][^2_16][^2_17][^2_18][^2_19][^2_20][^2_21][^2_22][^2_23][^2_24][^2_25][^2_26][^2_27][^2_28][^2_29][^2_30][^2_31][^2_32][^2_33][^2_34][^2_35][^2_36][^2_37][^2_38][^2_39][^2_40][^2_41]</span>

<div align="center">⁂</div>

[^2_1]: https://github.com/jtroo/kanata/blob/main/docs/setup-linux.md

[^2_2]: https://www.kernel.org/doc/html/v4.14/input/uinput.html

[^2_3]: https://www.reddit.com/r/rust/comments/w0zqk4/kanata_v105_an_advanced_keyboard_remapper_for/?tl=ko

[^2_4]: https://manpages.opensuse.org/Tumbleweed/keyd/keyd.1.en.html

[^2_5]: https://wiki.ultramarine-linux.org/en/usage/keyboard-cfg/

[^2_6]: https://man.archlinux.org/man/keyd.1.en

[^2_7]: https://www.antixforum.com/forums/topic/keyd-for-low-level-keyboard-mapping/

[^2_8]: https://github.com/jtroo/kanata/issues/1579

[^2_9]: https://www.reddit.com/r/KeyboardLayouts/comments/ualj0l/kanata_a_software_keyboard_remapper_for_windows/

[^2_10]: https://www.youtube.com/watch?v=sLWQ4Gx88h4

[^2_11]: https://www.reddit.com/r/linux/comments/5ypf9i/is_a_linux_with_only_wayland_no_x_possible_right/

[^2_12]: https://www.youtube.com/watch?v=xC7XAmsPs-8

[^2_13]: https://precondition.github.io/home-row-mods

[^2_14]: https://github.com/jtroo/kanata/issues/18

[^2_15]: https://github.com/jtroo/kanata/releases

[^2_16]: https://blog.zsa.io/layout-buffet-home-row-mods/

[^2_17]: https://www.reddit.com/r/swaywm/comments/i094ed/no_terminal_on_pure_wayland_x11_disabled/?tl=de

[^2_18]: https://www.reddit.com/r/archlinux/comments/rg364q/what_is_tty_in_arch_linux_and_how_does_it_work/

[^2_19]: https://www.reddit.com/r/ErgoMechKeyboards/comments/16d5lep/a_guide_to_home_row_mods/

[^2_20]: https://www.reddit.com/r/NobaraProject/comments/1cgt8io/how_do_i_permanently_switch_from_wayland_to_x11/

[^2_21]: https://stackoverflow.com/questions/14200782/run-console-to-tty-dynamically-on-linux

[^2_22]: https://callistaenterprise.se/blogg/teknik/2025/01/10/homerow-mods/

[^2_23]: https://github.com/alacritty/alacritty/issues/3340

[^2_24]: https://blog.devdata.com.br/como-usar-cada-tty-do-linux-para-um-servidor-ssh-diferente-sem-login-local/

[^2_25]: https://havn.blog/2024/03/03/a-good-way.html

[^2_26]: https://github.com/kovidgoyal/kitty/issues/1525

[^2_27]: https://stackoverflow.com/questions/4426280/what-do-pty-and-tty-mean

[^2_28]: https://sunaku.github.io/home-row-mods.html

[^2_29]: https://discussion.fedoraproject.org/t/preferred-method-to-re-map-modifiers-in-tty-console-sessions-i-e-caplock-ctrl/128834

[^2_30]: https://forum.manjaro.org/t/login-into-a-tty-console-fails-due-to-wrong-keyboad-mapping-what-to-do/169578

[^2_31]: https://github.com/jtroo/kanata/wiki/Avoid-using-sudo-on-Linux

[^2_32]: https://github.com/sezanzeb/input-remapper/issues/994

[^2_33]: https://bbs.archlinux.org/viewtopic.php?id=308351

[^2_34]: https://www.reddit.com/r/linux/comments/105ocez/if_you_want_to_remap_your_keyboard_keys_or_mouse/

[^2_35]: https://bbs.archlinux.org/viewtopic.php?id=107187

[^2_36]: https://man.archlinux.org/man/extra/keyd/keyd.1.en

[^2_37]: https://community.mnt.re/t/help-me-understand-this-udev-uinput-behaviour/2437

[^2_38]: https://www.reddit.com/r/linuxquestions/comments/17302fx/how_to_map_keyboard_key_to_send_a_string_of/

[^2_39]: https://www.reddit.com/r/linux/comments/ou3hi1/keyd_a_key_remapping_daemon_for_linux/

[^2_40]: https://www.reddit.com/r/rust/comments/w0zqk4/kanata_v105_an_advanced_keyboard_remapper_for/

[^2_41]: https://discourse.nixos.org/t/how-to-changing-tty-keyboard-layout/1954


---

## Complete Guide to keyd on Arch Linux

keyd is a powerful system-wide keyboard remapping daemon for Linux that works at the kernel level, making it perfect for Arch users who switch between GUI environments, Wayland, X11, and TTY consoles.[^3_1][^3_2]

### Installation on Arch

Install keyd from the official repositories:[^3_3]

```bash
sudo pacman -S keyd
```

Enable and start the service:

```bash
sudo systemctl enable keyd
sudo systemctl start keyd
```


### Basic Configuration Structure

All config files go in `/etc/keyd/` and must have a `.conf` extension. The basic structure is:[^3_4][^3_1]

```ini
[ids]
*

[main]
# Your key bindings here
```


### Understanding the [ids] Section

The `[ids]` section specifies which keyboards this config applies to:[^3_5][^3_4][^3_1]

```ini
# Match all keyboards
[ids]
*

# Match specific keyboard by ID (get IDs with: keyd monitor)
[ids]
0123:4567

# Match all EXCEPT specific keyboards
[ids]
* -0123:4567

# Match multiple specific keyboards
[ids]
046d:c52b
04d9:0024
```

Get your keyboard ID by running:

```bash
sudo keyd monitor
```


### The [main] Layer

The `[main]` layer is where you define your primary bindings. Each key is bound to itself by default.[^3_4][^3_1]

**Basic remapping:**

```ini
[main]
# Swap caps lock and escape
capslock = esc
esc = capslock

# Remap right alt to backspace
rightalt = backspace
```


## Key Overloading (Tap vs Hold)

keyd's most powerful feature is **overloading** - making a key do one thing when tapped, another when held.[^3_6][^3_1][^3_4]

### overload() Function

**Syntax:** `overload(<layer>, <tap_action>)`

```ini
[main]
# Caps lock: tap for escape, hold for control layer
capslock = overload(control, esc)
```

When you **tap** capslock, it sends `esc`. When you **hold** it, it activates the `control` layer with Control modifier active.[^3_1][^3_6][^3_5]

### overloadt() - With Timeout

**Syntax:** `overloadt(<layer>, <tap_action>, <timeout_ms>)`

```ini
[main]
# Must hold for 200ms to activate control layer
a = overloadt(control, a, 200)
```

This adds a visual delay when typing because keyd waits to see if you'll hold the key.[^3_4][^3_1]

### overloadt2() - Better for Fast Typing

**Syntax:** `overloadt2(<layer>, <tap_action>, <timeout_ms>)`

```ini
[main]
# Smarter timeout that considers other key presses
s = overloadt2(shift, s, 200)
```

This resolves as a tap immediately if another key is pressed before the timeout expires.[^3_1][^3_4]

### overloadi() - Idle Timeout (Best for Home Row Mods)

**Syntax:** `overloadi(<tap_action>, <hold_action>, <idle_timeout_ms>)`

```ini
[main]
# If typed within 150ms of last key: tap immediately
# Otherwise: wait for release or 200ms timeout
a = overloadi(a, overloadt2(control, a, 200), 150)
```

This eliminates typing lag by immediately resolving as a letter when typing fast.[^3_4][^3_1]

### lettermod() Macro - Simplified Home Row Mods

```
**Syntax:** `lettermod(<layer>, <key>, <idle_timeout>, <hold_timeout>)`
```

This is a built-in macro that combines `overloadi` and `overloadt2`:[^3_1]

```ini
[main]
# Home row mods made simple
a = lettermod(meta, a, 150, 200)
s = lettermod(alt, s, 150, 200)
d = lettermod(shift, d, 150, 200)
f = lettermod(control, f, 150, 200)

j = lettermod(control, j, 150, 200)
k = lettermod(shift, k, 150, 200)
l = lettermod(alt, l, 150, 200)
; = lettermod(meta, ;, 150, 200)
```


## Layers

Layers are alternative keymaps that activate when triggered.[^3_7][^3_4][^3_1]

### Basic Layer Syntax

```ini
[main]
capslock = layer(nav)

[nav]
h = left
j = down
k = up
l = right
```

Hold `capslock` to turn HJKL into arrow keys.[^3_4][^3_1]

### Layers with Modifiers

Add modifiers after a colon:

**Modifier codes:**

- **C** = Control
- **S** = Shift
- **A** = Alt
- **M** = Meta/Super
- **G** = AltGr

```ini
[main]
capslock = layer(capslock)

# Capslock acts as Control for everything...
[capslock:C]
# ...except these specific keys
j = down
k = up
h = left
l = right
```

Now `capslock+j` produces `down`, while `capslock+f` produces `C-f`.[^3_7][^3_1][^3_4]

### Composite Layers

Define layers that activate when **multiple** layers are active simultaneously:[^3_1][^3_4]

```ini
[main]
leftalt = layer(alt)
leftcontrol = layer(control)

[control:C]
[alt:A]

# Only active when BOTH control AND alt are pressed
[control+alt]
h = left
j = down
k = up
l = right
```

**Important:** Composite layers must be defined **after** their constituent layers.[^3_4][^3_1]

### Pre-defined Modifier Layers

keyd automatically defines these layers:[^3_1][^3_4]

```ini
[control:C]  # Activated by control keys
[shift:S]    # Activated by shift keys
[alt:A]      # Activated by left alt
[altgr:G]    # Activated by right alt
[meta:M]     # Activated by meta/super keys
```

You can add bindings to these directly:

```ini
[control]
j = down
k = up

[shift]
# Invert numbers and symbols
1 = !
2 = @
```


## Layer Actions

### layer(<layer>)

Hold to activate layer temporarily:[^3_1]

```ini
[main]
space = layer(nav)
```


### toggle(<layer>)

Toggle layer on/off with each press:[^3_1]

```ini
[main]
f12 = toggle(gaming)

[gaming]
# Your gaming-specific bindings
```


### oneshot(<layer>)

Layer stays active for **one** keypress, then deactivates:[^3_4][^3_1]

```ini
[main]
shift = oneshot(shift)
control = oneshot(control)
alt = oneshot(alt)
```

Now you can **tap** shift, then **tap** a letter to capitalize it without holding.[^3_2][^3_5][^3_1]

### swap(<layer>)

Immediately switches to a layer and stays there until you manually switch back:[^3_1]

```ini
[main]
f1 = swap(symbols)

[symbols]
f1 = swap(main)  # Switch back to main
```

**Warning:** Easy to get stuck in impossible-to-exit states. Use carefully.[^3_1]

## Macro Variants of Layer Actions

These execute a macro **before** changing layers:[^3_8][^3_1]

```ini
[main]
# Run macro, then activate layer
space = layerm(nav, macro(running nav layer))

# Run macro, then oneshot
shift = oneshotm(shift, macro(shift activated))

# Run macro before toggle
f12 = togglem(gaming, macro(game mode))
```


## Chording (Key Combinations)

Press multiple keys simultaneously to trigger an action:[^3_5][^3_4][^3_1]

```ini
[main]
# Press j and k together to get escape
j+k = esc

# Three-key chord
a+s+d = macro(Hello World)
```

**Chord timeout** (default 50ms) determines how close together keys must be pressed:

```ini
[global]
chord_timeout = 75
```


## Macros

Macros let you output sequences of keys or text.[^3_5][^3_4][^3_1]

### Basic Macro Syntax

**Syntax:** `macro(<token1> <token2> ...)`

```ini
[main]
# Type "Hello World"
f1 = macro(Hello space World)

# Multiple keypresses
f2 = macro(C-c 100ms C-v)
```

**Tokens:**

- **Text**: `hello`, `world`
- **Keys**: `space`, `enter`, `tab`, `esc`
- **Modifiers**: `C-c` (Ctrl+C), `M-t` (Meta+T), `S-a` (Shift+A), `A-f` (Alt+F)
- **Delays**: `100ms`, `500ms` (max 1024ms)
- **Unicode**: Any UTF-8 character (requires setup, see Unicode section)


### Macro Examples

```ini
[main]
# Email address
f3 = macro(user@example.com)

# Complex sequence
f4 = macro(C-a 50ms C-c 100ms M-tab 100ms C-v)

# With modifiers held
f5 = macro(S-hello space S-world)  # Types "HELLO WORLD"
```


### macro2() - With Repeat

**Syntax:** `macro2(<initial_delay>, <repeat_delay>, <macro>)`

```ini
[nav]
# Hold to continuously move left
h = macro2(400, 50, left)
```

Waits 400ms before first repeat, then repeats every 50ms.[^3_1]

### Macro Escaping

Split tokens to avoid special interpretation:

```ini
# This types "space" (the word)
f1 = macro(s pace)

# This types "3+5" (not key chords)
f2 = macro(3 + 5)
```


## Layouts (Alternative Letter Arrangements)

Layouts are special layers for changing the alphabet layout (like Dvorak, Colemak):[^3_4][^3_1]

```ini
[ids]
*

[dvorak:layout]
a = a
s = o
d = e
f = u
# ... full layout

[main]
f1 = setlayout(dvorak)
f2 = setlayout(main)
```

keyd ships with pre-made layouts in `/usr/share/keyd/layouts/`:[^3_1]

```ini
[ids]
*

include layouts/colemak

[main]
f1 = setlayout(colemak)
f2 = setlayout(main)
```


## Oneshot Modifiers

Make modifiers "sticky" - they stay active for one keypress after release:[^3_2][^3_5][^3_4][^3_1]

```ini
[main]
shift = oneshot(shift)
control = oneshot(control)
alt = oneshot(alt)
meta = oneshot(meta)
```

Now you can:

1. **Tap** shift
2. **Tap** a
3. Get capital 'A'

**With timeout:**

```ini
[global]
oneshot_timeout = 2000  # Deactivates after 2 seconds

[main]
shift = oneshot(shift)
```


## timeout() Action

Do different things based on how long you hold a key:[^3_1]

**Syntax:** `timeout(<quick_action>, <timeout_ms>, <long_action>)`

```ini
[main]
# Tap quickly: types 'a'
# Hold > 500ms: activates control layer
a = timeout(a, 500, layer(control))
```


## Global Settings

Set timing and behavior options in the `[global]` section:[^3_4][^3_1]

```ini
[global]
# Macro timings
macro_timeout = 600              # Delay before first macro repeat (ms)
macro_repeat_timeout = 50        # Delay between macro repeats (ms)
macro_sequence_timeout = 0       # Delay between keys in macro (microseconds)

# Chord settings
chord_timeout = 50               # Max time between chord keys (ms)
chord_hold_timeout = 0           # How long chord must be held (ms)

# Oneshot settings
oneshot_timeout = 0              # Oneshot expires after X ms (0 = never)

# Overload settings
overload_tap_timeout = 0         # Ignore tap if held > X ms (0 = never)

# LED indicator
layer_indicator = 0              # Turn on caps lock LED when layer active

# Modifier guard
disable_modifier_guard = 0       # Prevent phantom modifier taps
```


## Complete Home Row Mods Example

Combining everything for optimal home row mods:[^3_6][^3_4][^3_1]

```ini
[ids]
*

[global]
chord_timeout = 50

[main]
# Using lettermod for clean syntax
a = lettermod(meta, a, 150, 200)
s = lettermod(alt, s, 150, 250)
d = lettermod(shift, d, 150, 200)
f = lettermod(control, f, 150, 200)

j = lettermod(control, j, 150, 200)
k = lettermod(shift, k, 150, 200)
l = lettermod(alt, l, 150, 250)
; = lettermod(meta, ;, 150, 200)

# Caps as escape/control
capslock = overload(control, esc)
```


## Aliases

Create custom key names:[^3_4][^3_1]

```ini
[aliases]
# Create a "super" alias for both meta keys
super = leftmeta
super = rightmeta

[main]
super = layer(nav)
```


## Application-Specific Remapping

keyd can apply different bindings per application.[^3_9][^3_10][^3_11][^3_2]

### Setup

1. **Add yourself to keyd group:**
```bash
sudo usermod -aG keyd $USER
```

Log out and back in.

2. **Create** `~/.config/keyd/app.conf`:
```ini
[alacritty]
alt.] = C-tab
alt.[ = C-S-tab

[firefox*]
alt.] = C-tab
alt.[ = C-S-tab
alt.t = C-t

[code]
alt.w = C-w
alt.s = C-s
```

3. **Run the mapper:**
```bash
keyd-application-mapper -d
```

Add this to your window manager startup (e.g., `~/.config/hypr/hyprland.conf`):

```bash
exec-once = keyd-application-mapper -d
```


### Finding Window Classes

Run in verbose mode to see window information:

```bash
keyd-application-mapper -v
```

Or check the log:

```bash
tail -f ~/.config/keyd/app.log
```


### Application Config Syntax

```ini
# Match by class name
[chromium]
alt.] = C-tab

# Match by class AND title (with wildcards)
[kitty|*nvim*]
esc = capslock

# Match class with wildcards
[st-*]
alt.1 = macro(Inside st terminal)
```


## File Inclusion

Share common config across multiple keyboards:[^3_4][^3_1]

**`/etc/keyd/common`:**

```ini
[main]
capslock = overload(control, esc)
space = layer(nav)

[nav]
h = left
j = down
k = up
l = right
```

**`/etc/keyd/default.conf`:**

```ini
[ids]
*

include common

[main]
# Additional keyboard-specific bindings
```


## Unicode Support

To type Unicode characters, you need to set up keyd's compose file:[^3_4][^3_1]

```bash
# System-wide (requires root)
sudo cp /usr/share/keyd/keyd.compose /usr/share/X11/locale/en_US.UTF-8/Compose

# OR per-user
ln -s /usr/share/keyd/keyd.compose ~/.XCompose
```

Restart your applications. Then:

```ini
[main]
# Types ö
f1 = ö

# Types €
f2 = €
```

**Important:** Use US layout on your display server, and set your desired layout in keyd instead.[^3_1]

## Useful Commands

```bash
# Monitor keyboard events (see key names)
sudo keyd monitor

# List all valid key names
keyd list-keys

# Reload configuration after changes
sudo keyd reload

# Check config for errors
keyd check /etc/keyd/default.conf

# View logs
sudo journalctl -eu keyd

# Test a binding temporarily (resets on reload)
sudo keyd bind 'a = b'

# Reset all temporary bindings
sudo keyd bind reset
```


## Emergency Panic Sequence

If you misconfigure and can't type: press **Backspace + Escape + Enter** simultaneously to force keyd to terminate.[^3_2][^3_5][^3_4][^3_1]

## Practical Examples

### Example 1: Programmer's Layer

```ini
[ids]
*

[main]
capslock = layer(programmer)

[programmer:C]
# Navigation
h = left
j = down
k = up
l = right
u = home
i = pageup
o = pagedown
p = end

# Symbols (no need for shift)
1 = !
2 = @
3 = #
4 = $
5 = %
6 = ^
7 = &
8 = *
9 = (
0 = )
```


### Example 2: Invert Shift Behavior

```ini
[main]
1 = !
2 = @
3 = #
# ... etc

[shift]
1 = 1
2 = 2
3 = 3
# ... etc
```

Now numbers require shift, symbols don't.[^3_1]

### Example 3: Dual Control Behavior

```ini
[main]
# Tap once: oneshot control
# Tap twice: toggle control
# Hold: normal control
control = oneshot(control)

[control]
control = toggle(control)
```


### Example 4: Media Keys Layer

```ini
[main]
rightalt = layer(media)

[media]
h = volumedown
l = volumeup
k = mute
space = playpause
j = previoussong
; = nextsong
```


### Example 5: Command Execution

```ini
[main]
# Adjust brightness (runs shell commands as root user)
f5 = command(brightnessctl set 5%-)
f6 = command(brightnessctl set +5%)
```

**Warning:** Commands run as root. Use carefully.[^3_1]

## Debugging Tips

1. **Check logs for errors:**
```bash
sudo journalctl -eu keyd -f
```

2. **Verify key names:**
```bash
sudo keyd monitor
```

3. **Test bindings live:**
```bash
sudo keyd bind 'capslock = a'  # Test without editing config
```

4. **Check config syntax:**
```bash
keyd check /etc/keyd/default.conf
```


## Best Practices

1. **Start simple** - Begin with basic remaps before adding complex overloading
2. **Use lettermod()** for home row mods - it's optimized for fast typing[^3_1]
3. **Adjust timings per finger** - Pinkies are slower than index fingers
4. **Use oneshot modifiers** - They're ergonomic and reduce held keys[^3_5][^3_2]
5. **Backup your config** - Keep it in version control
6. **Test in isolated environment** - Make sure you can recover from bad configs
7. **Use layers strategically** - Group related functionality together
8. **Document your config** - Future you will thank present you

## Resources

- **Man page:** `man keyd`
- **GitHub:** https://github.com/rvaiya/keyd
- **Examples:** `/usr/share/keyd/` on your system
- **Community:** IRC channel \#keyd on OFTC

keyd is an extremely powerful tool that brings QMK-like functionality to any keyboard on your Arch system, working seamlessly across TTY, X11, and Wayland environments.
<span style="display:none">[^3_12][^3_13][^3_14][^3_15][^3_16][^3_17][^3_18][^3_19][^3_20][^3_21][^3_22][^3_23][^3_24][^3_25][^3_26][^3_27][^3_28][^3_29][^3_30][^3_31][^3_32][^3_33][^3_34][^3_35][^3_36][^3_37][^3_38][^3_39]</span>

<div align="center">⁂</div>

[^3_1]: https://man.archlinux.org/man/keyd.1.en

[^3_2]: https://github.com/rvaiya/keyd

[^3_3]: https://archlinux.org/packages/extra/x86_64/kmonad/

[^3_4]: https://man.archlinux.org/man/extra/keyd/keyd.1.en

[^3_5]: https://github.com/rvaiya/keyd/blob/master/README.md

[^3_6]: https://razzi.abuissa.net/guides/keyd

[^3_7]: https://man.uex.se/1/keyd

[^3_8]: https://github.com/Dygmalab/Kaleidoscope-Dygma/issues/4

[^3_9]: https://github.com/rvaiya/keyd/blob/master/docs/keyd-application-mapper.scdoc

[^3_10]: https://man.archlinux.org/man/extra/keyd/keyd-application-mapper.1.en

[^3_11]: https://man.archlinux.org/man/keyd-application-mapper.1.en

[^3_12]: https://gitea.yarmo.eu/yarmo/qmk_firmware/src/commit/2d4a69cfc6e0b182818f9b5eb3511ac879312351/docs/tap_hold.md

[^3_13]: https://github.com/joric/qmk/blob/master/docs/tap_hold.md

[^3_14]: https://www.reddit.com/r/olkb/comments/py0088/completely_lost_with_making_qmk_macros_and/

[^3_15]: https://docs.qmk.fm/tap_hold

[^3_16]: https://github.com/rvaiya/keyd/issues/718

[^3_17]: https://blog.zsa.io/per-key-dynamic-tapping-terms/

[^3_18]: https://jonny-wamsley.gitbook.io/the-ultimate-guide-to-keyboard-layers/ch-4-creating-layers/4.4-implementing-layers

[^3_19]: https://github.com/rvaiya/keyd/issues/608

[^3_20]: https://www.reddit.com/r/AsahiLinux/comments/1fj9xvt/my_maclike_keybindings_config_keyd/

[^3_21]: https://community.keyboard.io/t/macros-tapping-a-key-from-another-layer-in-the-same-physical-location/1554

[^3_22]: https://www.reddit.com/r/ErgoMechKeyboards/comments/178cg4x/tap_hold_triggers/

[^3_23]: https://razzi.abuissa.net/guides/keyd/

[^3_24]: https://www.youtube.com/watch?v=6x6qNDhQK-I

[^3_25]: https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-OneShot.html

[^3_26]: https://precondition.github.io/home-row-mods

[^3_27]: https://docs.qmk.fm/one_shot_keys

[^3_28]: https://github.com/jtroo/kanata/blob/main/cfg_samples/home-row-mod-advanced.kbd

[^3_29]: https://www.reddit.com/r/ErgoMechKeyboards/comments/ieegqa/implementing_home_row_mods/

[^3_30]: https://manpages.ubuntu.com/manpages/questing/man1/keyd-application-mapper.1.html

[^3_31]: https://github.com/rvaiya/keyd/issues/437

[^3_32]: https://manpages.debian.org/experimental/keyd-application-mapper/keyd-application-mapper.1.en.html

[^3_33]: https://www.reddit.com/r/linux/comments/ou3hi1/keyd_a_key_remapping_daemon_for_linux/

[^3_34]: https://www.reddit.com/r/ErgoMechKeyboards/comments/16d5lep/a_guide_to_home_row_mods/

[^3_35]: https://www.youtube.com/watch?v=7j2TqT7EU10

[^3_36]: https://forum.ultimatehackingkeyboard.com/t/usecase-one-shot-macro-tutorial/304

[^3_37]: https://havn.blog/2024/03/03/a-good-way.html

[^3_38]: https://betterjavacode.com/programming/custom-protocol-mapper-keycloak

[^3_39]: https://community.keyboard.io/t/macro-that-respects-one-shot-modifier-keys/6163


---

## Yes, Both keyd and Kanata Support Custom Layers

Both tools absolutely support creating multiple layers just like programmable keyboards (QMK/ZMK). This is one of their most powerful features, letting you add navigation layers, symbol layers, number pads, function keys, and more to any regular keyboard.[^4_1][^4_2][^4_3][^4_4][^4_5][^4_6]

## Layers in keyd

I already covered keyd layers extensively in the previous guide, but here's a quick recap of the key concepts:[^4_6][^4_7]

### Layer Activation Methods

**1. Temporary layer (hold):**

```ini
[main]
space = layer(nav)

[nav]
h = left
j = down
k = up
l = right
```

**2. Toggle layer (on/off):**

```ini
[main]
f12 = toggle(numpad)

[numpad]
u = 7
i = 8
o = 9
# ... etc
```

**3. Oneshot layer (single key):**

```ini
[main]
shift = oneshot(shift)
```

**4. Swap to layer:**

```ini
[main]
f1 = swap(colemak)
```


## Layers in Kanata

Kanata uses a different syntax but offers similar (and even more advanced) layer capabilities.[^4_2][^4_3][^4_8][^4_1]

### Basic Kanata Layer Structure

```lisp
;; Define which keys exist on your keyboard
(defsrc
  esc  1    2    3    4    5    6    7    8    9    0    -    =    bspc
  tab  q    w    e    r    t    y    u    i    o    p    [    ]    \
  caps a    s    d    f    g    h    j    k    l    ;    '    ret
  lsft z    x    c    v    b    n    m    ,    .    /    rsft
  lctl lmet lalt           spc            ralt rmet rctl
)

;; Base layer (first layer is always active on startup)
(deflayer base
  esc  1    2    3    4    5    6    7    8    9    0    -    =    bspc
  tab  q    w    e    r    t    y    u    i    o    p    [    ]    \
  @cap a    s    d    f    g    h    j    k    l    ;    '    ret
  lsft z    x    c    v    b    n    m    ,    .    /    rsft
  lctl lmet lalt           @spc           ralt rmet rctl
)

;; Navigation layer
(deflayer nav
  _    _    _    _    _    _    _    _    _    _    _    _    _    _
  _    _    _    _    _    _    home pgdn pgup end  _    _    _    _
  _    _    _    _    _    _    left down up   rght _    _    _
  _    _    _    _    _    _    _    _    _    _    _    _
  _    _    _              _              _    _    _
)

;; Symbols layer
(deflayer symbols
  _    _    _    _    _    _    _    _    _    _    _    _    _    _
  _    S-1  S-2  S-3  S-4  S-5  S-6  S-7  S-8  S-9  S-0  _    _    _
  _    _    _    _    _    _    _    _    _    _    _    _    _
  _    _    _    _    _    _    _    _    _    _    _    _
  _    _    _              _              _    _    _
)

;; Aliases define what @ keys do
(defalias
  cap (tap-hold 200 200 esc lctl)
  spc (tap-hold 200 200 spc (layer-while-held nav))
)
```

**Key points:**

- `_` (underscore) = transparent key, passes through to lower layer[^4_9][^4_8][^4_1]
- Layers stack on top of each other[^4_1][^4_2]
- The first `deflayer` is the startup layer[^4_8][^4_1]
- Maximum of 25 layers allowed[^4_1]


### Layer Activation Actions in Kanata

**1. layer-while-held** - Hold key to temporarily activate layer:[^4_10][^4_11][^4_2][^4_1]

```lisp
(defalias
  nav (layer-while-held navigation)
)

(deflayer base
  @nav
)
```

**2. layer-toggle** - Press to toggle layer on/off:[^4_10][^4_1]

```lisp
(defalias
  num (layer-toggle numbers)
)
```

**3. layer-switch** - Permanently switch to another layer:[^4_12][^4_10][^4_1]

```lisp
(defalias
  qw (layer-switch qwerty)
  col (layer-switch colemak)
)
```

**4. tap-hold with layer** - Tap for key, hold for layer:[^4_9][^4_2][^4_1]

```lisp
(defalias
  ;; Tap space = space, hold space = nav layer
  spc (tap-hold 200 200 spc (layer-while-held nav))
  
  ;; Tap caps = esc, hold caps = control layer
  cap (tap-hold 200 200 esc (layer-while-held control))
)
```


## Practical Layer Examples

### Example 1: Vim-Style Navigation Layer (Kanata)

```lisp
(defsrc
  caps a    s    d    f    g    h    j    k    l    spc
)

(deflayer base
  @cap a    s    d    f    g    h    j    k    l    @spc
)

;; Hold caps for navigation
(deflayer nav
  _    _    _    _    _    _    left down up   rght _
)

;; Hold space for more navigation
(deflayer nav2
  _    home pgdn pgup end  _    _    _    _    _    _
)

(defalias
  cap (layer-while-held nav)
  spc (tap-hold 200 200 spc (layer-while-held nav2))
)
```

Hold `caps` → hjkl become arrow keys[^4_13][^4_11][^4_14][^4_9]
Hold `space` → asdf become home/pgdn/pgup/end

### Example 2: Symbol Layer (Kanata)

```lisp
(deflayer base
  tab  q    w    e    r    t    y    u    i    o    p
  @sym a    s    d    f    g    h    j    k    l    ;
)

(deflayer symbols
  _    S-1  S-2  S-3  S-4  S-5  S-6  S-7  S-8  S-9  S-0
  _    !    @    #    $    %    ^    &    *    \(   \)
)

(defalias
  sym (layer-while-held symbols)
)
```

Hold the sym key → top row becomes `!@#$%^&*()`, second row has easy access to symbols.[^4_15][^4_16][^4_17][^4_9]

### Example 3: Number Pad Layer (keyd)

```ini
[main]
rightalt = layer(numpad)

[numpad]
# Right hand becomes numpad
u = 7
i = 8
o = 9
j = 4
k = 5
l = 6
m = 1
, = 2
. = 3
space = 0

# Math operators on left
y = kp+
h = kp-
n = kp*
/ = kp/
```

Hold right alt → right hand becomes a full numpad with operators.[^4_18][^4_19][^4_6]

### Example 4: Function Keys Layer (Kanata)

```lisp
(deflayer base
  caps 1    2    3    4    5    6    7    8    9    0
)

(deflayer func
  _    f1   f2   f3   f4   f5   f6   f7   f8   f9   f10
)

(defalias
  cap (layer-while-held func)
)
```

Hold caps → number row becomes F1-F10.[^4_20][^4_19][^4_4]

### Example 5: Composite Layers - Both Hands (Kanata)

Activate a special layer when **two** layer keys are held simultaneously:[^4_2][^4_1]

```lisp
(deflayer base
  lalt           spc
)

(defalias
  alt (layer-while-held altlayer)
  spc (layer-while-held spacelayer)
)

;; Hold just alt
(deflayer altlayer
  _              _
)

;; Hold just space
(deflayer spacelayer
  _              _
)

;; Hold BOTH alt AND space = composite layer
(deflayer altlayer+spacelayer
  ;; Special actions only when both are held
)
```

This is powerful for accessing rarely-used functions without dedicating a full key.[^4_2][^4_1]

### Example 6: Multiple Layer Switching (Kanata)

Create a "layer selector" layer:[^4_10]

```lisp
(defalias
  ;; Hold right alt to access layer switcher
  comp (layer-while-held changer)
  
  ;; Layer switch aliases
  qw (layer-switch qwerty)
  col (layer-switch colemak)
  gam (layer-switch gaming)
)

(deflayer base
  @comp
)

;; Hold right alt, then press 1/2/3 to switch layouts
(deflayer changer
  _ @qw @col @gam _
)
```

Hold right alt, tap 1 for QWERTY, 2 for Colemak, 3 for gaming mode.[^4_10]

## Advanced Layer Features

### Layer Priorities (Kanata)

Layers stack from bottom to top. Higher layers override lower layers:[^4_1][^4_2]

```
Layer 3: symbols     (highest priority)
Layer 2: navigation
Layer 1: numbers
Layer 0: base        (lowest priority)
```

If Layer 2 and Layer 3 are both active, Layer 3's mappings take precedence.[^4_2][^4_1]

### Transparent Keys vs Pass-through

- **`_` (transparent)**: Falls through to the layer below[^4_8][^4_9][^4_1]
- **`XX`**: Explicitly blocks the key (does nothing)[^4_8][^4_1]

```lisp
(deflayer nav
  _    ;; Falls through to base layer
  XX   ;; Key does nothing on this layer
)
```


### Leader Keys (Kanata)

Create Vim-like leader sequences:[^4_3][^4_11]

```lisp
(defseq leader-w (w))
(defseq leader-q (q))

(defalias
  ldr (leader-timeout 1000)
)

(deflayer base
  @ldr
)

;; Press leader then 'w' within 1 second
(deflayer leader-w
  ;; Maximize window
  (macro M-f10)
)

;; Press leader then 'q' within 1 second  
(deflayer leader-q
  ;; Close window
  (macro M-f4)
)
```


### Multi-Layer Navigation Example (Full Config)

A complete example showing how to combine multiple layers for a productivity setup:[^4_11][^4_8][^4_2]

```lisp
(defcfg
  process-unmapped-keys yes
)

(defsrc
  caps a    s    d    f    g    h    j    k    l    ;    '
  lsft z    x    c    v    b    n    m    ,    .    /    rsft
  lctl lmet lalt           spc            ralt rmet rctl
)

(deflayer base
  @cap @a   @s   @d   @f   g    h    @j   @k   @l   @;   '
  lsft z    x    c    v    b    n    m    ,    .    /    rsft
  lctl lmet lalt           @spc           @sym @num rctl
)

;; Navigation layer via caps
(deflayer nav
  _    _    _    _    _    _    left down up   rght _    _
  _    home pgdn pgup end  _    _    _    _    _    _    _
  _    _    _              _              _    _    _
)

;; Symbols layer via right alt
(deflayer symbols
  _    S-1  S-2  S-3  S-4  S-5  S-6  S-7  S-8  S-9  S-0  _
  _    !    @    #    $    %    ^    &    *    \(   \)   _
  _    _    _              _              _    _    _
)

;; Number pad via right meta
(deflayer numbers
  _    _    _    _    _    _    7    8    9    kp+  _    _
  _    _    _    _    _    _    4    5    6    kp-  _    _
  _    _    _              0              _    _    _
)

(defalias
  ;; Home row mods
  a (tap-hold 200 200 a lmet)
  s (tap-hold 200 250 s lalt)
  d (tap-hold 200 200 d lsft)
  f (tap-hold 200 200 f lctl)
  
  j (tap-hold 200 200 j rctl)
  k (tap-hold 200 200 k rsft)
  l (tap-hold 200 250 l ralt)
  ; (tap-hold 200 200 ; rmet)
  
  ;; Layer access
  cap (tap-hold 200 200 esc (layer-while-held nav))
  spc (tap-hold 200 200 spc (layer-while-held nav))
  sym (layer-while-held symbols)
  num (layer-while-held numbers)
)
```

This gives you:

- **Home row mods** on base layer
- **Navigation** via caps or space hold
- **Symbols** via right alt hold
- **Number pad** via right meta hold[^4_14][^4_11][^4_9][^4_2]


## Comparison: keyd vs Kanata for Layers

| Feature | keyd | Kanata |
| :-- | :-- | :-- |
| **Max layers** | Unlimited | 25 |
| **Syntax** | INI-style, simple | Lisp-like, more verbose |
| **Layer stacking** | Yes | Yes |
| **Composite layers** | Yes (e.g., `[control+alt]`) | Yes (e.g., `altlayer+spacelayer`) |
| **Leader sequences** | No | Yes |
| **Per-app layers** | Yes (via app mapper) | No built-in |
| **Learning curve** | Easier | Steeper |
| **Documentation** | Good man pages | Extensive GitHub docs |

## Tips for Designing Layers

1. **Purpose-specific layers**: Create dedicated layers for symbols, navigation, numbers, media, etc.[^4_4][^4_20][^4_9][^4_2]
2. **Bilateral access**: Put layer activation keys on one hand, layer content on the other[^4_21][^4_2]
3. **Vim-inspired navigation**: hjkl for arrows is muscle memory for many developers[^4_13][^4_11][^4_14][^4_9]
4. **Group related functions**: Put Home/End near PgUp/PgDn[^4_18][^4_2]
5. **Math operators with numpad**: If you have a number layer, add `+-*/` nearby[^4_19][^4_18]
6. **Start simple**: Begin with one navigation layer before adding symbols, numbers, etc.[^4_22][^4_4]
7. **Mirror QMK layouts**: If you've used QMK keyboards, replicate those layers[^4_15][^4_9][^4_2]

Both keyd and Kanata turn any regular keyboard into a fully programmable keyboard with as many layers as you need. The layer system is exactly like QMK/ZMK firmware on custom keyboards, but implemented in software at the kernel level.[^4_5][^4_23][^4_3][^4_6][^4_1]
<span style="display:none">[^4_24][^4_25][^4_26][^4_27][^4_28][^4_29][^4_30][^4_31][^4_32][^4_33][^4_34][^4_35][^4_36][^4_37][^4_38][^4_39][^4_40][^4_41]</span>

<div align="center">⁂</div>

[^4_1]: https://github.com/jtroo/kanata/wiki/Configuration-guide

[^4_2]: https://ryan.himmelwright.net/post/building-34-key-layout/

[^4_3]: https://docs.rs/crate/kanata/1.0.8

[^4_4]: https://dygma.com/blogs/stories/from-noob-to-pro-master-the-layers-like-a-boss-😎

[^4_5]: https://github.com/jtroo/kanata

[^4_6]: https://man.archlinux.org/man/keyd.1.en

[^4_7]: https://man.archlinux.org/man/extra/keyd/keyd.1.en

[^4_8]: https://balkian.com/p/kanata-advanced-keyboard-configuration/

[^4_9]: https://getreuer.info/posts/keyboards/symbol-layer/index.html

[^4_10]: https://github.com/jtroo/kanata/discussions/1451

[^4_11]: https://www.reddit.com/r/vim/comments/1i26map/i_remapped_my_keyboard_to_navigate_desktop/

[^4_12]: https://github-wiki-see.page/m/greqov/sowa/wiki/How-to-setup-kanata-for-Latin-\&-Cyrillic-layouts

[^4_13]: https://www.reddit.com/r/neovim/comments/187fp6j/simple_kanata_config_key_remapping_for_vimstyle/?tl=ko

[^4_14]: https://www.linkedin.com/posts/saumyadipjana2003_kanata-productivity-keyboard-activity-7374663299902205952-nc5l

[^4_15]: https://colemakmods.github.io/ergonomic-mods/symbols.html

[^4_16]: https://www.reddit.com/r/olkb/comments/9bkyr9/qmk_esc_when_tapped_mo_layer_switch_when_held/?tl=ja

[^4_17]: https://dev.to/shanu-kumawat/boost-your-linux-productivity-remapping-useless-keys-with-kanata-3ih5

[^4_18]: https://hackaday.io/project/185358/log/218466-keyboard-layout-and-layers

[^4_19]: https://www.reddit.com/r/olkb/comments/q4mhpr/fkeys_a_numpad_symbols_navigation_keys_whatre_on/

[^4_20]: https://www.youtube.com/watch?v=6lZzSNGUc2M

[^4_21]: https://precondition.github.io/home-row-mods

[^4_22]: https://shom.dev/start/using-kanata-to-remap-any-keyboard/

[^4_23]: https://github.com/rvaiya/keyd

[^4_24]: https://forum.systemcrafters.net/t/using-kanata-to-remap-your-keyboard/524?page=2

[^4_25]: https://pkg.go.dev/github.com/rszyma/kanata-tray

[^4_26]: https://www.reddit.com/r/KeyboardLayouts/comments/1k3s118/i_really_like_my_kanata_layout_but_dont_have/

[^4_27]: https://www.youtube.com/watch?v=xTFAbuvcF0A

[^4_28]: https://github.com/jtroo/kanata/discussions/1649

[^4_29]: https://github.com/jtroo/kanata/blob/main/cfg_samples/simple.kbd

[^4_30]: https://www.reddit.com/r/krita/comments/wpwnuv/kanatakrita_2_layers_mouseless_quick_switch_macro/

[^4_31]: https://www.youtube.com/watch?v=jvJ3f4HHiMY

[^4_32]: https://www.monsgeek.com/faq/ways-to-remap-layer-switching-key-and-tap-hold-keys-in-via/

[^4_33]: https://github.com/jtroo/kanata/issues/459

[^4_34]: https://www.reddit.com/r/KeyboardLayouts/comments/1n2k87s/kanata_config_help_needed/

[^4_35]: https://zmk.dev/docs/keymaps/behaviors/layers

[^4_36]: https://forum.ultimatehackingkeyboard.com/t/strategy-for-dedicated-symbols-and-numbers-layer-combos/1469/8

[^4_37]: https://www.onlydust.com/repositories/kmonad/kmonad/issues/1006

[^4_38]: https://codingfearlessly.com/vim-putting-arrows-to-use

[^4_39]: https://forum.systemcrafters.net/t/using-kanata-to-remap-your-keyboard/524?page=2\&rut=86746feb490871dd4134564ddb7f26d2bc30831e1350369298ea1068c87208be

[^4_40]: https://raw.githubusercontent.com/conventoangelo/OverKeys/main/docs/advanced/layer-switching.md

[^4_41]: https://github.com/jakeru/kanata-conf

