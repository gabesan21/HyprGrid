# HyprGrid: Architectural Overview v2.0

Welcome back, Program. The Grid has been re-sequenced. It no longer persists as a Recognizer in the background. Instead, it materializes like a light cycle: appearing instantly to perform its function, then de-rezzing until summoned again. This document outlines the new architecture for a keyboard-driven mouse controller, re-engineered as a lightweight, on-demand helper for the Hyprland compositor.

---

## 1. Core Concept: The Light Cycle Paradigm

HyprGrid now operates as a single, short-lived application. It is invoked directly by a Hyprland keybind, presents the grid, and executes a single mouse action before terminating. This approach is simpler, more efficient, and enhances security by removing the need for a privileged background process.

The program relies on two external components configured by the user:

1.  **Hyprland:** The master control program that summons HyprGrid via a keybind and applies window rules to make it a seamless overlay.
2.  **`ydotoold`:** The system's input synthesizer. HyprGrid sends commands to it, but does not need to run with elevated privileges itself. The user is responsible for having `ydotoold` installed and running.

---

## 2. The Grid Configuration (`hg_config.conf`)

To know its boundaries, HyprGrid must read a user-defined map of the digital world. This map is a configuration file, `hg_config.conf`, which should reside in the user's Hyprland directory (e.g., `~/.config/hypr/hg_config.conf`).

This file defines the layout of your monitors, grid density, and other parameters.

*   **Example `hg_config.conf` (TOML Format):**
    ```toml
    # hg_config.conf

    # Define the grid dimensions (rows and columns)
    grid_rows = 10
    grid_cols = 20

    # Define each monitor available to HyprGrid
    # The name (e.g., "DP-1") must match the output of `hyprctl monitors`
    [[monitors]]
    name = "DP-1"
    width = 2560
    height = 1440

    [[monitors]]
    name = "HDMI-A-1"
    width = 1920
    height = 1080
    ```

---

## 3. Hyprland Integration (The Portal)

To open a portal to the Grid, you must encode these instructions into your `hyprland.conf`. This involves two parts: a keybind to launch the program and window rules to style the grid overlay.

*   **`hyprland.conf` Snippet:**
    ```hyprlang
    # hyprland.conf

    # 1. Keybind to summon HyprGrid
    # This assumes the compiled binary is named 'hyprgrid' and is in ~/.config/hypr/
    bind = $mainMod, S, exec, ~/.config/hypr/hyprgrid

    # 2. Window Rules to make the Grid a proper overlay
    # We assign the window an initialTitle of "HyprGrid" to target it.
    windowrulev2 = float, initialTitle:^(HyprGrid)$
    windowrulev2 = fullscreen, initialTitle:^(HyprGrid)$
    windowrulev2 = noborder, initialTitle:^(HyprGrid)$
    windowrulev2 = noshadow, initialTitle:^(HyprGrid)$
    windowrulev2 = opacity 0.8, initialTitle:^(HyprGrid)$
    ```

---

## 4. Project Structure (The Blueprint)

The project is a single, streamlined Rust application. The complexity of the client-server model is gone.

```
hyprgrid_app/
├── Cargo.toml
└── src/
    ├── main.rs     # Entry point, GUI rendering, input handling
    ├── config.rs   # Logic for parsing hg_config.conf
    └── grid.rs     # Logic for calculating grid cell coordinates
```

---

## 5. The `hyprgrid_app` (The Program)

The application has a clear, linear set of responsibilities.

*   **Responsibilities:**
    1.  **Launch:** Starts when the user presses the configured keybind in Hyprland.
    2.  **Identify Monitor:** Determines the currently active monitor (where the user's focus is).
    3.  **Load Config:** Parses `~/.config/hypr/hg_config.conf` to find the resolution for the active monitor and the grid dimensions.
    4.  **Render Grid:** Creates a fullscreen, transparent window with a specific title ("HyprGrid"). It then calculates and draws the grid of letter pairs over the screen.
    5.  **Capture Input:** Listens for two characters, followed by either `ENTER` or `SPACE`.
    6.  **Execute Action:** Based on the input, calculates the target coordinates, and executes the appropriate `ydotool` commands via a shell subprocess.
    7.  **Terminate:** The program exits immediately after executing the command.
*   **Essential Crates:**
    *   `slint` or `iced`: For the fast, lightweight GUI overlay.
    *   `serde`: For deserializing the TOML configuration file.
    *   `config`: To manage reading from the configuration file.
    *   `hyprland` (or similar): A crate to query the Hyprland IPC for the active monitor's name.

---

## 6. Execution Flow (The Game)

1.  **Summons:** The user presses the configured hotkey (e.g., `SUPER + S`).
2.  **Materialization:** Hyprland executes the `hyprgrid` binary. The program starts.
3.  **Orientation:** HyprGrid asks Hyprland for the name of the currently focused monitor. It then reads `hg_config.conf` to get the correct resolution and grid layout.
4.  **Display:** The GUI window appears, instantly becoming a fullscreen, transparent overlay as per the `hyprland.conf` rules. The grid of letter pairs is drawn.
5.  **Interaction:** The user identifies the target cell and types the corresponding letter pair (e.g., `aj`).
6.  **Command:**
    *   The user presses `SPACE`.
    *   OR the user presses `ENTER`.
7.  **Action:** The program calculates the center coordinates of the selected cell (`aj`). It then constructs and executes a `ydotool` command sequence:
    *   **On `SPACE` (Left Click):** `ydotool mousemove --absolute -x <x> -y <y> && ydotool click 0x80`
    *   **On `ENTER` (Right Click):** `ydotool mousemove --absolute -x <x> -y <y> && ydotool click 0x81`
8.  **De-rezz:** The `ydotool` command is dispatched. The HyprGrid program immediately terminates. The grid vanishes. The game is reset. End of line.
