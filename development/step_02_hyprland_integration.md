# Step 2: Hyprland Integration

## Objective
Implement communication with Hyprland to identify which monitor is currently active (where the user's focus is).

## Why This Matters
HyprGrid needs to know which monitor to display the grid on. In a multi-monitor setup, the grid should appear on whichever monitor the user is currently using, not on all monitors or a random one.

## Hyprland IPC
Hyprland provides an IPC (Inter-Process Communication) interface that allows external programs to:
- Query the state of the compositor
- Get information about monitors, workspaces, windows
- Execute commands

## What Information to Retrieve

### Active Monitor
You need to determine:
- Which monitor currently has keyboard/mouse focus
- The monitor's name (e.g., "DP-1", "HDMI-A-1")
- Optionally: the monitor's current position (x, y offset) if using absolute positioning

### How Hyprland Identifies Monitors
- Each monitor has a unique name assigned by the system
- These names must match exactly what's in `hg_config.conf`
- The active monitor is typically the one containing the focused window or cursor

## Implementation Approaches

### Using a Hyprland Crate
If using a Rust Hyprland library:
- Look for methods to get the active monitor
- Understand the data structures returned
- Handle cases where no monitor is active (edge case)
- Consider caching if multiple queries are needed

### Direct IPC Communication
If implementing IPC manually:
- Connect to Hyprland's socket
- Send the appropriate query command
- Parse the JSON response
- Extract the active monitor name

## Technical Considerations

### Timing
- Query Hyprland EARLY in the application lifecycle
- Before rendering the GUI
- Before loading the corresponding monitor config

### Error Handling
Handle these scenarios:
- Hyprland is not running (user not in Hyprland session)
- IPC socket not accessible
- Unexpected response format
- No active monitor (theoretical edge case)
- Monitor name doesn't match any in config file

### Fallback Behavior
When the active monitor can't be determined:
- Should the app use the first monitor in the config?
- Should it exit with an error?
- Should it allow the user to specify manually?
- Document your decision

## Matching with Configuration

### Cross-Referencing
After getting the monitor name from Hyprland:
- Look it up in the parsed configuration
- Retrieve the width and height for that monitor
- Handle the case where the monitor isn't in the config

### Configuration Mismatch Scenarios
- Monitor is connected but not in config → Error or warning?
- Monitor in config but not connected → Should be ignored
- Monitor name has changed → Helpful error message

## Multi-Monitor Considerations

### Current Focus
- The active monitor is usually where the cursor is
- Or where the currently focused window is located
- Hyprland tracks this automatically

### Future Enhancements
Consider how the design might support:
- Displaying grids on all monitors simultaneously
- Allowing the user to switch monitors while the grid is active
- Remembering the last-used monitor

## Testing Considerations

### Test Scenarios
- Single monitor setup
- Multi-monitor setup with different resolutions
- Monitor connected but turned off
- Running outside of Hyprland
- Monitor disconnected while app is starting

### Mock Testing
Since testing might not always be in Hyprland:
- Consider allowing mock/test mode
- Ability to specify a monitor name via CLI argument
- Environment variable for testing

## Success Criteria
This step is complete when:
- You can successfully query Hyprland for the active monitor name
- The monitor name is correctly matched with the configuration
- Appropriate error messages are shown when something goes wrong
- The solution works reliably in multi-monitor setups
- Edge cases (Hyprland not running, unknown monitor) are handled gracefully
