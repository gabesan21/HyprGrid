# Step 4: GUI Overlay

## Objective
Create a fullscreen, transparent overlay window that displays the grid with letter-pair labels over the entire active monitor.

## Window Requirements

### Initial Window Properties
The window must be created with:
- **Title**: "HyprGrid" (exact match required for window rules)
- **Fullscreen**: Covers the entire monitor
- **Transparent/Semi-transparent**: User can see through to desktop
- **Frameless**: No window decorations, borders, or title bar
- **Topmost**: Above all other windows

### Hyprland Window Rules
The `hyprland.conf` rules will handle:
- Making the window float
- Setting it to fullscreen
- Removing borders and shadows
- Setting opacity (e.g., 0.8)

Your application just needs to:
- Set the initial title correctly
- Request fullscreen size
- Be prepared to receive those properties

## GUI Framework Considerations

### Using Iced
If using the iced GUI framework:
- Understand how to create a window with specific settings
- How to set the window title
- How to make the window fullscreen
- How to handle transparency (may depend on compositor)

### Window Lifecycle
- Create the window
- Render the grid
- Keep the window open until user completes interaction
- Close immediately after action is executed

## Rendering the Grid

### What to Draw

#### Grid Lines (Optional)
- Horizontal lines separating rows
- Vertical lines separating columns
- Should be subtle, not distracting
- Color: contrasting with typical desktop backgrounds

#### Cell Labels (Required)
For each cell:
- Display the two-letter identifier
- Positioned within the cell (center or top-left)
- Font size appropriate for cell size
- Must be readable

### Visual Styling

#### Colors
Consider:
- **Background**: Transparent or semi-transparent dark overlay
- **Grid lines**: Subtle gray or white
- **Labels**: High contrast (white on dark, or with background box)
- **Highlight**: Optional - highlight hovered/focused cells

#### Typography
- **Font**: Monospace font for consistent spacing
- **Size**: Scaled based on cell size
- **Weight**: Bold enough to be readable
- **Anti-aliasing**: Ensure crisp rendering

#### Opacity
- Background overlay: 0.3-0.5 (subtle presence)
- Grid lines: 0.5-0.7 (visible but not dominant)
- Labels: 1.0 (fully opaque, maximum readability)

## Coordinate System

### Screen Space
- Origin (0, 0) is typically top-left
- X increases to the right
- Y increases downward
- All grid calculations should align with this

### Drawing Primitives
For each cell, you'll need to:
- Draw text at a specific position
- Optionally draw rectangles for cell borders
- Optionally fill cell backgrounds

## Performance Considerations

### Rendering Speed
- Must appear instantly when invoked
- Initial render should be < 100ms
- 60 FPS is not necessary (static display)
- Redraw only when needed (user input)

### Resource Usage
- Keep it lightweight
- No animations or transitions needed
- Static grid after initial render
- Clean up resources on exit

## Interaction States

### Initial State
- Grid is displayed
- All cells are visible
- Waiting for user input
- No cell is highlighted

### During Input
Optional enhancements:
- Highlight the cell as letters are typed
- Show which cells match the first letter
- Visual feedback for valid/invalid input

### Before Exit
- Grid may stay visible until action completes
- Or disappear immediately when action is triggered
- Choose based on user experience

## Transparency Challenges

### Compositor Support
- Hyprland supports transparency via window rules
- Application may need to request ARGB visual
- Test that transparency works as expected

### Readability
- Labels must be readable on any background
- Consider adding a semi-opaque background box behind each label
- Or use a contrasting outline/shadow on text

### Click-Through Prevention
- The window should capture input, not pass through
- Grid overlay must be interactive
- User should not accidentally click through to underlying windows

## Resolution Independence

### Scaling
The grid should work on any resolution:
- Calculate cell sizes dynamically
- Scale font sizes based on cell dimensions
- Minimum font size to ensure readability
- Maximum grid density to prevent cramped labels

### Multi-Monitor
- Only render on the active monitor
- Respect monitor position and offset
- Don't assume monitor starts at (0, 0)

## Testing Considerations

### Visual Tests
- Does the grid appear correctly?
- Are all labels visible and readable?
- Is the transparency appropriate?
- Do window rules apply correctly?

### Different Resolutions
Test on:
- 1920×1080 (Full HD)
- 2560×1440 (QHD)
- 3840×2160 (4K)
- Ultrawide (3440×1440)

### Grid Densities
- Small grid: 5×5
- Medium grid: 10×20
- Large grid: 20×30
- Ensure labels scale appropriately

## Success Criteria
This step is complete when:
- A fullscreen overlay appears on the active monitor
- The window has the correct title ("HyprGrid")
- The grid is rendered with all letter-pair labels visible
- Labels are readable on various backgrounds
- Hyprland window rules are applied correctly
- The window stays on top and captures input
- Performance is acceptable (renders quickly)
