# Step 1: Configuration Parsing

## Objective
Implement the ability to read and parse the HyprGrid configuration file (`hg_config.conf`) from the user's Hyprland directory.

## Configuration File Location
The configuration file should be located at:
- Default path: `~/.config/hypr/hg_config.conf`
- Must expand the `~` to the actual HOME directory path
- Handle cases where the file doesn't exist with clear error messages

## Configuration Structure
The config file uses TOML format and contains:

### Grid Dimensions
- `grid_rows`: Number of rows in the grid (e.g., 10)
- `grid_cols`: Number of columns in the grid (e.g., 20)
- These define how many cells the screen will be divided into

### Monitor Definitions
- Array of monitor configurations: `[[monitors]]`
- Each monitor has:
  - `name`: Must match exactly what Hyprland reports (e.g., "DP-1", "HDMI-A-1")
  - `width`: Horizontal resolution in pixels
  - `height`: Vertical resolution in pixels

## Implementation Considerations

### Data Structure
Create a configuration structure that:
- Holds the grid dimensions as integers
- Contains a collection of monitor definitions
- Is deserializable from TOML format
- Provides methods to find a specific monitor by name

### Error Handling
Handle these error cases gracefully:
- Configuration file not found
- Invalid TOML syntax
- Missing required fields (grid_rows, grid_cols)
- No monitors defined
- Invalid values (negative dimensions, zero dimensions)

### Validation
After parsing, validate that:
- Grid dimensions are reasonable (e.g., between 2 and 50)
- At least one monitor is defined
- Monitor dimensions are positive integers
- Monitor names are not empty

## Design Principles

### Separation of Concerns
- Keep configuration parsing separate from the rest of the application
- Configuration module should only be responsible for reading and validating the config
- No GUI or Hyprland-specific logic in this module

### User-Friendly Errors
When something goes wrong, provide helpful messages:
- Tell the user exactly which file you tried to read
- Explain what's wrong with the configuration
- Suggest how to fix common issues
- Point to example configurations

### Future Extensibility
Design the configuration structure to allow for future additions:
- Additional grid settings (colors, opacity, fonts)
- Per-monitor grid dimensions
- Custom keybindings
- Action modifiers

## Testing Considerations

### What to Test
- Valid configuration files are parsed correctly
- Invalid TOML is rejected with clear errors
- Missing files produce helpful error messages
- All required fields are validated
- Monitor lookup by name works correctly

### Edge Cases
- Empty configuration file
- Configuration with only monitors, no grid dimensions
- Duplicate monitor names
- Monitors with same name but different resolutions
- Unicode characters in monitor names
- Very large grid dimensions

## Success Criteria
This step is complete when:
- You can successfully read and parse a valid `hg_config.conf` file
- Invalid configurations produce clear, helpful error messages
- You can look up a specific monitor configuration by name
- The configuration structure is well-documented and easy to use
