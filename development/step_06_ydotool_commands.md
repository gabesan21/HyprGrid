# Step 6: ydotool Command Execution

## Objective
Execute ydotool commands to move the mouse cursor to the selected coordinates and perform the requested click action.

## ydotool Overview

### What is ydotool?
- A command-line tool for synthesizing input events
- Works at the kernel level (like xdotool but for Wayland)
- Requires `ydotoold` daemon to be running
- User must have appropriate permissions

### Why ydotool?
- Wayland compositors don't allow direct input injection for security
- ydotool works with Wayland (including Hyprland)
- Alternative to X11's xdotool

## Required Commands

### Mouse Movement
Move the cursor to absolute coordinates:
```
ydotool mousemove --absolute -x <X> -y <Y>
```

Parameters:
- `--absolute`: Use screen coordinates (not relative movement)
- `-x <X>`: The target X coordinate in pixels
- `-y <Y>`: The target Y coordinate in pixels

### Mouse Clicks

**Left Click:**
```
ydotool click 0x80
```
- `0x80` is the code for left mouse button

**Right Click:**
```
ydotool click 0x81
```
- `0x81` is the code for right mouse button

### Combined Action
To move and click in one operation:
```
ydotool mousemove --absolute -x <X> -y <Y> && ydotool click 0x80
```

Using `&&` ensures the click only happens if movement succeeds.

## Command Construction

### Building the Command String
Given:
- Target coordinates (x, y) from grid calculations
- User's action choice (SPACE or ENTER)

Construct the command:
1. Start with `ydotool mousemove --absolute`
2. Add `-x <x_coordinate>`
3. Add `-y <y_coordinate>`
4. Add `&&`
5. Add `ydotool click 0x80` (left) or `0x81` (right)

### Example
For cell "aj" at coordinates (960, 540) with left click:
```
ydotool mousemove --absolute -x 960 -y 540 && ydotool click 0x80
```

## Command Execution

### Spawning a Process
Execute the command using:
- Shell subprocess (e.g., `sh -c "command"`)
- Or execute ydotool directly with arguments (no shell needed)

### Execution Method
Two approaches:

**Approach 1: Shell command**
- Run: `sh -c "ydotool mousemove --absolute -x 960 -y 540 && ydotool click 0x80"`
- Simpler to construct
- Shell handles the `&&` logic

**Approach 2: Multiple direct calls**
- Call ydotool twice directly
- First: mousemove with arguments
- Second: click with argument
- No shell needed, potentially more reliable

### Waiting for Completion
Should the application wait for ydotool to finish?
- Movement is nearly instant
- Click is nearly instant
- Probably wait to ensure action completes
- But timeout after a reasonable period (e.g., 1 second)

## Error Handling

### Possible Failures

**ydotoold not running:**
- Error: "Failed to connect to ydotool socket"
- The `ydotoold` daemon must be running
- User should start it with: `systemctl start ydotool`

**Permission denied:**
- User doesn't have permission to use ydotool
- Needs to be in correct group or run daemon as user

**Command not found:**
- ydotool not installed
- User needs to install ydotool package

**Invalid coordinates:**
- Coordinates outside screen bounds
- Should never happen if grid calculations are correct
- Defensive check before execution

### Error Messages
When ydotool fails:
- Capture stderr output
- Display to user with helpful context
- Suggest solutions:
  - "Is ydotoold running? Try: systemctl start ydotool"
  - "Is ydotool installed? Try: sudo pacman -S ydotool"
  - "Check permissions for ydotool"

## Timing Considerations

### Execution Delay
There may be a slight delay:
- Between mousemove and click
- Usually < 10ms, imperceptible
- No need to add artificial delays

### Application Termination
When should HyprGrid exit?
- **Immediately after spawning command**: Fastest, but might miss errors
- **After command completes**: Safer, ensures action succeeded
- **Hybrid**: Fire and forget for move, wait for click confirmation

Recommended: Wait for command completion (< 100ms typically)

## Testing Considerations

### Without ydotool
How to test without breaking your desktop?
- **Dry run mode**: Print command instead of executing
- **Mock mode**: Simulate success/failure
- **Logging**: Log all commands being executed

### Verification
After execution:
- Did the cursor actually move?
- Did the click happen?
- Hard to verify programmatically
- User testing is essential

### Edge Cases
- Coordinates at (0, 0) - top-left corner
- Coordinates at max (screen edge)
- Very rapid repeated invocations
- System under heavy load

## Security Considerations

### Input Sanitization
Even though coordinates come from your own calculations:
- Validate they're positive integers
- Ensure they're within screen bounds
- Prevent command injection (use argument arrays, not string concatenation)

### Command Injection Prevention
Never do this:
```
command = "ydotool mousemove -x " + user_input
```

Always use proper argument passing or escape properly.

In this case, coordinates come from your calculations, not user input, but good practice anyway.

## Dependencies

### User Responsibilities
Document clearly that users must:
1. Install ydotool package
2. Start/enable ydotoold service
3. Ensure they have permission to use it

### Checking Prerequisites
Optional: Check if ydotool is available before running:
- Check if command exists: `which ydotool`
- Check if daemon is running: `systemctl status ydotool`
- Provide helpful error if not found

## Success Criteria
This step is complete when:
- You can successfully construct ydotool commands with correct syntax
- Commands execute and move the mouse cursor accurately
- Left and right clicks work as expected
- Errors from ydotool are captured and displayed helpfully
- The application handles ydotool failures gracefully
- Command execution is fast (< 100ms)
