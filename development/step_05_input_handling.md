# Step 5: Input Handling

## Objective
Capture and interpret user keyboard input to select a grid cell and determine the desired action (left click or right click).

## Input Sequence

### Expected Pattern
The user will type:
1. **First character**: A letter (a-z)
2. **Second character**: A letter (a-z)
3. **Action key**: Either SPACE or ENTER

### Example Interactions
- User types `a`, then `j`, then `SPACE` → Left click on cell "aj"
- User types `h`, then `k`, then `ENTER` → Right click on cell "hk"

## Input State Management

### State Machine
The application goes through these states:
1. **Waiting for first character**
   - Any valid letter (a-z) is accepted
   - Invalid keys are ignored
   - Store the first character

2. **Waiting for second character**
   - Any valid letter (a-z) is accepted
   - Invalid keys are ignored
   - Store the second character
   - Now have a complete cell identifier

3. **Waiting for action**
   - SPACE → Left click
   - ENTER → Right click
   - Other keys → ignored or reset?

### Reset Conditions
What happens if the user makes a mistake?
- **ESC key**: Cancel and close the application
- **Backspace**: Go back one state (undo last character)
- **Invalid cell**: Reject and reset, or ignore?
- **Timeout**: Close after X seconds of inactivity?

## Keyboard Event Handling

### Event Types
You'll need to handle:
- Key press events
- Character input events
- Ignore key repeats (if holding a key)

### Case Sensitivity
- Should uppercase letters be accepted?
- Convert to lowercase automatically?
- The grid labels are typically lowercase

### Special Keys to Handle
- **a-z**: Valid grid input
- **SPACE**: Trigger left click
- **ENTER/RETURN**: Trigger right click
- **ESC**: Cancel and exit
- **BACKSPACE**: Optional - undo last input
- All other keys: Ignore

## Validation

### Cell Identifier Validation
After capturing two letters:
- Does this combination exist in the grid?
- Look up in the grid mapping (from Step 3)
- If invalid: show error or ignore?

### Error Handling
If user types an invalid cell (e.g., "zz" when grid only goes to "tt"):
- Option 1: Ignore and reset input
- Option 2: Show visual feedback (flash, color change)
- Option 3: Beep or sound notification
- Option 4: Display error message

## Visual Feedback

### Display Current Input
Show the user what they've typed:
- Display the characters as they're entered
- Position: center of screen? top corner?
- Clear, large font
- Example: "a_" while waiting for second character

### Cell Preview
Optional enhancement:
- Highlight matching cells as user types
- After first letter: highlight all cells starting with that letter
- After second letter: highlight the selected cell
- Helps user confirm their selection before clicking

### Status Indicator
Show what state the application is in:
- "Type 2 letters..." initially
- "Press SPACE or ENTER" after two letters are entered
- "Clicking..." while executing

## User Experience Considerations

### Speed vs. Safety
- Should there be a confirmation step?
- Or execute immediately on SPACE/ENTER?
- Balance between speed (fewer keystrokes) and safety (prevent mistakes)

### Undo Capability
Allow the user to correct mistakes:
- Backspace removes last character
- ESC cancels entire operation
- Without undo, user must be very careful

### Timeout Behavior
Should the grid auto-close if inactive?
- Pro: Prevents stuck overlays
- Con: Might close while user is thinking
- Suggestion: 30-60 second timeout, or no timeout

## Integration with GUI

### Event Loop
The GUI framework (iced) will provide keyboard events:
- Subscribe to keyboard events
- Process events in the update/message handler
- Update application state based on input

### State Updates
Each keystroke should:
- Update internal state
- Trigger a re-render if needed (to show feedback)
- Potentially trigger the action execution

## Edge Cases

### Rapid Input
What if user types very quickly?
- Ensure all keystrokes are captured
- Don't skip characters due to rendering
- Buffer input if necessary

### Multiple Keys Simultaneously
What if user holds multiple keys?
- Should be rare with normal typing
- Handle key repeats (ignore if already processed)

### Focus Loss
What if the window loses focus somehow?
- Should probably close/cancel
- Or regain focus automatically

### Non-English Keyboards
- Stick to a-z letter keys (works internationally)
- Avoid assuming specific keyboard layout
- Don't rely on key position, use key character

## Testing Considerations

### Test Scenarios
- Valid input: "ab" + SPACE
- Valid input: "xy" + ENTER
- Invalid cell: "zz" + SPACE (if zz doesn't exist)
- ESC at various stages
- Backspace usage
- Typing very fast
- Typing very slowly

### Automated Testing
Can you test input handling without GUI?
- Mock keyboard events
- Test state machine logic separately
- Verify state transitions

## Success Criteria
This step is complete when:
- The application correctly captures two-letter input
- SPACE and ENTER are recognized as action triggers
- Invalid input is handled gracefully
- ESC properly cancels the operation
- User receives appropriate visual feedback
- The input state machine works reliably
- Edge cases (rapid input, invalid cells) are handled
