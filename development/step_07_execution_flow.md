# Step 7: Complete Execution Flow

## Objective
Orchestrate all components into a cohesive application that executes from start to finish following the Light Cycle Paradigm.

## The Light Cycle Paradigm

### Core Principle
HyprGrid is NOT a persistent daemon. It:
- Materializes when summoned (keybind pressed)
- Performs a single action
- De-rezzes immediately (terminates)

### Lifecycle
1. **Invocation**: User presses Hyprland keybind
2. **Materialization**: Process starts, window appears
3. **Interaction**: User selects cell and action
4. **Execution**: Mouse action is performed
5. **Termination**: Process exits

Total lifetime: Typically 1-3 seconds

## Application Flow

### Phase 1: Initialization (~ 50-100ms)

**Step 1.1: Startup**
- Application entry point (main function)
- Parse command-line arguments (if any)
- Initialize logging/error handling

**Step 1.2: Query Active Monitor**
- Connect to Hyprland IPC
- Request active monitor information
- Extract monitor name
- Handle error if Hyprland unavailable

**Step 1.3: Load Configuration**
- Read `~/.config/hypr/hg_config.conf`
- Parse TOML content
- Validate configuration
- Handle error if config invalid/missing

**Step 1.4: Match Monitor**
- Look up active monitor in config
- Retrieve monitor width and height
- Handle error if monitor not in config

**Step 1.5: Calculate Grid**
- Get grid dimensions from config
- Calculate cell positions
- Generate letter-pair mappings
- Build lookup table: letters → coordinates

### Phase 2: Display (~ 10-50ms)

**Step 2.1: Create Window**
- Initialize GUI framework (iced)
- Create window with title "HyprGrid"
- Request fullscreen mode
- Set up transparency

**Step 2.2: Render Grid**
- Draw grid overlay
- Render all cell labels
- Display initial state
- Wait for window to appear

**Step 2.3: Hyprland Window Rules**
- Hyprland automatically applies window rules
- Window becomes floating, fullscreen, borderless
- Opacity set to configured value
- Window stays on top

### Phase 3: Interaction (variable, user-dependent)

**Step 3.1: Wait for Input**
- Listen for keyboard events
- Display input prompt/status
- Update UI with visual feedback

**Step 3.2: Capture First Letter**
- User types first character
- Validate it's a valid letter
- Store in state
- Update UI to show progress
- Optional: Highlight matching cells

**Step 3.3: Capture Second Letter**
- User types second character
- Validate it's a valid letter
- Store in state
- Form complete cell identifier
- Validate cell exists in grid
- Optional: Highlight selected cell

**Step 3.4: Capture Action**
- User presses SPACE or ENTER
- Determine action type:
  - SPACE → Left click
  - ENTER → Right click
- Proceed to execution

**Alternative: Cancellation**
- User presses ESC
- Skip to Phase 4 (termination)
- No action is performed

### Phase 4: Execution (~ 10-50ms)

**Step 4.1: Lookup Coordinates**
- Use cell identifier to find coordinates
- Retrieve (x, y) from grid mapping
- Verify coordinates are valid

**Step 4.2: Construct Command**
- Build ydotool command string
- Include coordinates
- Include click type (left/right)

**Step 4.3: Execute Command**
- Spawn ydotool subprocess
- Wait for completion
- Capture any errors
- Handle failures gracefully

**Step 4.4: Verify (Optional)**
- Check if command succeeded
- Log any errors
- Display error to user if needed

### Phase 5: Termination (~ 10ms)

**Step 5.1: Cleanup**
- Close GUI window
- Release resources
- Close file handles/connections

**Step 5.2: Exit**
- Return exit code (0 for success, non-zero for error)
- Process terminates
- Grid disappears from screen

## Total Timeline

Typical successful execution:
```
0ms     - Start
50ms    - Config loaded, grid calculated
100ms   - Window displayed
[wait]  - User interaction (1-2 seconds)
+50ms   - Action executed
+60ms   - Process terminated
```

Total: ~2-3 seconds from invocation to termination

## Error Handling Strategy

### Fail Fast
If something goes wrong early:
- Display clear error message
- Don't show the grid
- Exit immediately
- User can try again

### Graceful Degradation
For non-critical errors:
- Log the error
- Continue if possible
- Inform user but don't block

### Error Scenarios

**Startup Errors (exit immediately):**
- Hyprland not running
- Config file not found
- Config file invalid
- Active monitor not in config
- Unable to create window

**Runtime Errors (show message, then exit):**
- Invalid cell identifier
- ydotool command failed
- Unexpected keyboard input error

**Cancellation (normal exit):**
- User pressed ESC
- Timeout reached
- Lost focus (optional)

## State Management

### Application State
Track the following state:
- Current input buffer (0-2 characters)
- Selected cell coordinates
- Action type (left/right click)
- Status (waiting, executing, error, done)

### State Transitions
```
Start → Loading → Ready → FirstChar → SecondChar → ActionReady → Executing → Done
                    ↓        ↓           ↓              ↓
                    ↓────── Cancelled ──────────────────┴→ Done
```

### Concurrent Execution
No concurrency needed:
- Linear, single-threaded flow
- One task at a time
- Simplifies implementation

## Integration Testing

### End-to-End Test
Simulate the complete flow:
1. Start application
2. Mock Hyprland IPC response
3. Load test configuration
4. Simulate keyboard input
5. Capture ydotool command (don't execute)
6. Verify command is correct
7. Check clean termination

### Manual Testing
Real-world testing:
1. Run in actual Hyprland session
2. Invoke via keybind
3. Select various cells
4. Test both left and right click
5. Test ESC cancellation
6. Test error scenarios

## Performance Goals

### Responsiveness
- Grid appears in < 100ms from invocation
- Input response is immediate (< 10ms)
- Action executes in < 100ms
- Total overhead (excluding user time) < 300ms

### Resource Usage
- Small memory footprint (< 50MB)
- Minimal CPU usage
- No memory leaks
- Clean resource cleanup

## User Experience

### Speed
- The faster, the better
- Users want instant response
- Minimize delays between steps

### Reliability
- Must work every time
- Failed actions are frustrating
- Clear feedback when things go wrong

### Simplicity
- Minimal keystrokes (2 letters + action)
- Intuitive grid layout
- Clear visual feedback

## Logging and Debugging

### What to Log
- Startup: monitor name, resolution, grid size
- Errors: detailed error messages with context
- Actions: cell selected, coordinates, action type
- Timing: performance metrics for optimization

### Log Levels
- **Error**: Critical failures
- **Warn**: Non-critical issues
- **Info**: Normal operation milestones
- **Debug**: Detailed flow for troubleshooting

### Log Location
- stderr for errors/warnings
- stdout for info (or silent by default)
- Optional: log file in `~/.config/hypr/hyprgrid.log`

## Success Criteria
This step is complete when:
- All components work together seamlessly
- The application can be invoked via Hyprland keybind
- Users can successfully select cells and perform clicks
- The application terminates cleanly after each action
- Error cases are handled gracefully
- Performance meets goals (< 100ms display time)
- The Light Cycle Paradigm is fully realized
- Manual testing shows reliable operation
- You're ready for real-world usage

## End of Line
When all seven steps are complete, HyprGrid will be fully operational. The Grid will be online, ready to serve its users with keyboard-driven precision.
