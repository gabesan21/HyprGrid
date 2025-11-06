# Step 3: Grid Calculations

## Objective
Implement the mathematical logic to divide the screen into a grid and calculate the coordinates for each cell.

## The Grid System

### Conceptual Overview
The screen (monitor) is divided into a grid of cells:
- The number of rows and columns comes from the config
- Each cell represents a clickable area
- Each cell is labeled with a unique two-letter combination

### Example
For a 10×20 grid on a 1920×1080 monitor:
- 10 rows = 108 pixels per row (1080 ÷ 10)
- 20 columns = 96 pixels per column (1920 ÷ 20)
- Total cells = 200 (10 × 20)

## Letter Assignment Algorithm

### Two-Letter Labeling
Each cell needs a unique identifier made of two letters:
- First letter: typically represents the row
- Second letter: typically represents the column
- Example sequence: aa, ab, ac, ... az, ba, bb, ...

### Letter Set
Using the English alphabet (26 letters):
- Maximum cells with 2 letters = 26 × 26 = 676 cells
- Should be sufficient for reasonable grid sizes
- Consider using home row letters first for ergonomics

### Ordering Strategy
Two common approaches:
1. **Row-major**: aa, ab, ac... (left to right, top to bottom)
2. **Column-major**: aa, ba, ca... (top to bottom, left to right)

Consider which is more intuitive for users.

### Home Row Optimization
The architecture mentions home row considerations:
- Home row letters: a, s, d, f, g, h, j, k, l
- These are easiest to type
- Should the most accessible cells use home row letters?
- Trade-off: ergonomics vs. alphabetical simplicity

## Coordinate Calculation

### Cell Boundaries
For each cell, calculate:
- Top-left corner (x, y)
- Bottom-right corner (x, y)
- Width and height

### Formula
For a cell at grid position (row, col):
- Cell width = monitor_width ÷ grid_cols
- Cell height = monitor_height ÷ grid_rows
- Top-left X = col × cell_width
- Top-left Y = row × cell_height

### Click Target
When a cell is selected, where should the mouse click?
- **Center of cell**: Most intuitive and forgiving
- Center X = top_left_x + (cell_width ÷ 2)
- Center Y = top_left_y + (cell_height ÷ 2)

### Pixel Precision
Consider:
- Integer rounding (cells might not divide evenly)
- Off-by-one errors at edges
- Ensuring all cells sum to the full screen size

## Data Structures

### Grid Representation
You need a structure to store:
- The letter pair for each cell
- The click coordinates for each cell
- Efficient lookup: letter pair → coordinates

### Lookup Performance
- Will be used once per invocation
- Performance is not critical
- Simple HashMap/Dictionary is sufficient
- Key: letter pair (String)
- Value: coordinates (x, y tuple)

## Visual Layout Considerations

### Label Positioning
Each cell needs to display its letter pair:
- Where in the cell should the label appear?
- Center? Top-left corner?
- Font size relative to cell size
- Ensure labels are readable on all grid densities

### Grid Lines
Should you draw borders between cells?
- Helps users see cell boundaries
- May be cluttered on high-density grids
- Consider making this configurable

### Color and Visibility
- Labels must be visible on any background
- Consider contrasting colors or background boxes
- Semi-transparent overlay might affect readability

## Edge Cases and Considerations

### Small Grids
- 2×2 grid (4 cells): aa, ab, ba, bb
- Very large click targets
- Simple and fast

### Large Grids
- 20×30 grid (600 cells)
- Very small cells and labels
- May require zooming or subdivision
- Is there a practical maximum?

### Non-Square Grids
- 5×20 grid (wide and short cells)
- 20×5 grid (tall and narrow cells)
- Algorithm should handle any aspect ratio

### Ultrawide Monitors
- 3840×1080 resolution
- Very wide cells if using equal divisions
- Should work without special cases

## Testing Considerations

### Validation Tests
- All cells have unique letter pairs
- All cells are within screen boundaries
- Cell centers are correctly calculated
- No overlapping cells

### Known Values
Test with specific examples:
- 1920×1080 with 10×10 grid
- Verify cell (0,0) = aa = center at (96, 54)
- Verify cell (9,9) = correct letter and position

### Boundary Conditions
- Cell at (0, 0) - top-left
- Cell at (max_row, max_col) - bottom-right
- Ensure no coordinates are negative or exceed screen size

## Success Criteria
This step is complete when:
- You can generate a complete mapping of letter pairs to coordinates
- The calculations are accurate for any monitor size and grid dimensions
- Letter assignments are logical and consistent
- The algorithm handles edge cases (very small/large grids)
- You can efficiently look up coordinates given a letter pair
