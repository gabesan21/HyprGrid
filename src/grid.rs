// Grid calculation module for HyprGrid
//
// This module implements the mathematical logic to divide the screen into a grid
// and calculate the coordinates for each cell.

use std::collections::HashMap;

/// Represents the coordinates and dimensions of a grid cell
#[derive(Debug, Clone, PartialEq)]
pub struct CellCoordinates {
    /// X coordinate of the top-left corner
    pub x: u32,
    /// Y coordinate of the top-left corner
    pub y: u32,
    /// Width of the cell in pixels
    pub width: u32,
    /// Height of the cell in pixels
    pub height: u32,
}

impl CellCoordinates {
    /// Calculate the center point of the cell (for mouse click target)
    pub fn center(&self) -> (u32, u32) {
        (
            self.x + self.width / 2,
            self.y + self.height / 2,
        )
    }
}

/// Represents a single cell in the grid
#[derive(Debug, Clone)]
pub struct GridCell {
    /// Two-letter label for this cell (e.g., "aa", "ab", "sd")
    #[allow(dead_code)]
    pub label: String,
    /// Coordinates and dimensions of the cell
    pub coordinates: CellCoordinates,
    /// Grid position (row, column)
    #[allow(dead_code)]
    pub grid_position: (u32, u32),
}

/// The complete grid structure with cell mappings
#[derive(Debug)]
pub struct Grid {
    /// Number of rows in the grid
    pub rows: u32,
    /// Number of columns in the grid
    pub cols: u32,
    /// Monitor dimensions
    #[allow(dead_code)]
    pub monitor_width: u32,
    #[allow(dead_code)]
    pub monitor_height: u32,
    /// HashMap for efficient lookup: letter pair -> GridCell
    cells: HashMap<String, GridCell>,
}

impl Grid {
    /// Create a new grid with the specified dimensions
    ///
    /// # Arguments
    /// * `rows` - Number of rows in the grid
    /// * `cols` - Number of columns in the grid
    /// * `monitor_width` - Width of the monitor in pixels
    /// * `monitor_height` - Height of the monitor in pixels
    ///
    /// # Returns
    /// A complete Grid with all cells calculated and labeled
    ///
    /// # Example
    /// ```no_run
    /// use hyprgrid::grid::Grid;
    ///
    /// let grid = Grid::new(10, 20, 1920, 1080);
    /// if let Some(cell) = grid.get_cell("aa") {
    ///     let (x, y) = cell.coordinates.center();
    ///     println!("Cell 'aa' center: ({}, {})", x, y);
    /// }
    /// ```
    pub fn new(rows: u32, cols: u32, monitor_width: u32, monitor_height: u32) -> Self {
        let mut cells = HashMap::new();
        let labels = generate_letter_labels(rows, cols);

        let cell_width = monitor_width / cols;
        let cell_height = monitor_height / rows;

        let mut label_index = 0;

        // Generate cells in row-major order (left to right, top to bottom)
        for row in 0..rows {
            for col in 0..cols {
                let label = labels[label_index].clone();
                label_index += 1;

                let coordinates = CellCoordinates {
                    x: col * cell_width,
                    y: row * cell_height,
                    width: cell_width,
                    height: cell_height,
                };

                let cell = GridCell {
                    label: label.clone(),
                    coordinates,
                    grid_position: (row, col),
                };

                cells.insert(label, cell);
            }
        }

        Grid {
            rows,
            cols,
            monitor_width,
            monitor_height,
            cells,
        }
    }

    /// Get a cell by its letter label
    ///
    /// # Arguments
    /// * `label` - The two-letter label (e.g., "aa", "sd")
    ///
    /// # Returns
    /// `Some(&GridCell)` if the label exists, `None` otherwise
    pub fn get_cell(&self, label: &str) -> Option<&GridCell> {
        self.cells.get(label)
    }

    /// Get the total number of cells in the grid
    pub fn total_cells(&self) -> usize {
        self.cells.len()
    }

    /// Get all cell labels in a sorted order
    #[allow(dead_code)]
    pub fn get_all_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = self.cells.keys().cloned().collect();
        labels.sort();
        labels
    }

    /// Check if a label exists in the grid
    #[allow(dead_code)]
    pub fn has_label(&self, label: &str) -> bool {
        self.cells.contains_key(label)
    }
}

/// Generate letter labels using home row priority
///
/// This function creates two-letter labels for grid cells using a home row priority
/// ordering. The home row letters (asdfghjkl) are used first for better ergonomics.
///
/// # Arguments
/// * `rows` - Number of rows in the grid
/// * `cols` - Number of columns in the grid
///
/// # Returns
/// A vector of letter labels in row-major order
///
/// # Letter Priority
/// Home row letters (9 letters): a, s, d, f, g, h, j, k, l
/// Extended set (remaining 17 letters): b, c, e, i, m, n, o, p, q, r, t, u, v, w, x, y, z
///
/// # Panics
/// Panics if the number of cells exceeds 676 (26×26 limit)
fn generate_letter_labels(rows: u32, cols: u32) -> Vec<String> {
    let total_cells = (rows * cols) as usize;

    // Home row letters (most ergonomic, used first)
    let home_row = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];

    // Remaining letters to reach 26 total
    let extended = ['b', 'c', 'e', 'i', 'm', 'n', 'o', 'p', 'q', 'r', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    // Combine home row and extended letters
    let mut letter_set = Vec::with_capacity(26);
    letter_set.extend_from_slice(&home_row);
    letter_set.extend_from_slice(&extended);

    assert!(
        total_cells <= 676,
        "Grid too large: {} cells exceeds maximum of 676 (26×26)",
        total_cells
    );

    let mut labels = Vec::with_capacity(total_cells);

    // Generate labels in row-major order
    for i in 0..total_cells {
        let first_idx = i / 26;
        let second_idx = i % 26;
        let label = format!("{}{}", letter_set[first_idx], letter_set[second_idx]);
        labels.push(label);
    }

    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_generation_small_grid() {
        let labels = generate_letter_labels(2, 2);
        assert_eq!(labels.len(), 4);
        assert_eq!(labels[0], "aa");
        assert_eq!(labels[1], "as");
        assert_eq!(labels[2], "ad");
        assert_eq!(labels[3], "af");
    }

    #[test]
    fn test_letter_generation_home_row_priority() {
        let labels = generate_letter_labels(1, 10);
        // First 9 should use home row letters
        assert_eq!(labels[0], "aa");
        assert_eq!(labels[1], "as");
        assert_eq!(labels[2], "ad");
        assert_eq!(labels[8], "al");
        // 10th should start using extended letters
        assert_eq!(labels[9], "ab");
    }

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 20, 1920, 1080);
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 20);
        assert_eq!(grid.total_cells(), 200);
    }

    #[test]
    fn test_cell_coordinates_simple() {
        // Simple case: 10×10 grid on 1000×1000 screen
        let grid = Grid::new(10, 10, 1000, 1000);

        // Cell at (0,0) should be at top-left
        let cell = grid.get_cell("aa").unwrap();
        assert_eq!(cell.coordinates.x, 0);
        assert_eq!(cell.coordinates.y, 0);
        assert_eq!(cell.coordinates.width, 100);
        assert_eq!(cell.coordinates.height, 100);
        assert_eq!(cell.grid_position, (0, 0));
    }

    #[test]
    fn test_cell_center_calculation() {
        let coords = CellCoordinates {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        let (cx, cy) = coords.center();
        assert_eq!(cx, 50);
        assert_eq!(cy, 50);
    }

    #[test]
    fn test_realistic_monitor() {
        // Test with real monitor dimensions: 1920×1080, 10×20 grid
        let grid = Grid::new(10, 20, 1920, 1080);

        let cell = grid.get_cell("aa").unwrap();
        assert_eq!(cell.coordinates.width, 96);  // 1920 / 20
        assert_eq!(cell.coordinates.height, 108); // 1080 / 10

        let (cx, cy) = cell.coordinates.center();
        assert_eq!(cx, 48);  // 96 / 2
        assert_eq!(cy, 54);  // 108 / 2
    }

    #[test]
    fn test_cell_lookup() {
        let grid = Grid::new(5, 5, 1000, 1000);

        assert!(grid.has_label("aa"));
        assert!(grid.has_label("as"));
        assert!(!grid.has_label("zz"));

        let cell = grid.get_cell("aa");
        assert!(cell.is_some());

        let invalid = grid.get_cell("zz");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_row_major_ordering() {
        // 2×3 grid should have cells in order: aa, as, ad (row 0), af, ag, ah (row 1)
        let grid = Grid::new(2, 3, 600, 400);

        let cell_aa = grid.get_cell("aa").unwrap();
        assert_eq!(cell_aa.grid_position, (0, 0));

        let cell_as = grid.get_cell("as").unwrap();
        assert_eq!(cell_as.grid_position, (0, 1));

        let cell_ad = grid.get_cell("ad").unwrap();
        assert_eq!(cell_ad.grid_position, (0, 2));

        let cell_af = grid.get_cell("af").unwrap();
        assert_eq!(cell_af.grid_position, (1, 0));
    }

    #[test]
    fn test_all_cells_within_bounds() {
        let grid = Grid::new(10, 20, 1920, 1080);

        for label in grid.get_all_labels() {
            let cell = grid.get_cell(&label).unwrap();
            assert!(cell.coordinates.x < grid.monitor_width);
            assert!(cell.coordinates.y < grid.monitor_height);
            assert!(cell.coordinates.x + cell.coordinates.width <= grid.monitor_width);
            assert!(cell.coordinates.y + cell.coordinates.height <= grid.monitor_height);
        }
    }

    #[test]
    fn test_unique_labels() {
        let grid = Grid::new(15, 15, 1920, 1080);
        let labels = grid.get_all_labels();
        let unique_count = labels.len();
        let total_count = grid.total_cells();

        assert_eq!(unique_count, total_count);
        assert_eq!(total_count, 225);
    }

    #[test]
    #[should_panic(expected = "Grid too large")]
    fn test_grid_too_large() {
        // 27×27 = 729 cells, exceeds 676 limit
        generate_letter_labels(27, 27);
    }

    #[test]
    fn test_edge_case_maximum_grid() {
        // 26×26 = 676 cells, exactly at limit
        let labels = generate_letter_labels(26, 26);
        assert_eq!(labels.len(), 676);
    }

    #[test]
    fn test_non_square_grid() {
        // Test with non-square grid
        let grid = Grid::new(5, 20, 1920, 1080);
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.cols, 20);
        assert_eq!(grid.total_cells(), 100);

        let cell = grid.get_cell("aa").unwrap();
        assert_eq!(cell.coordinates.width, 96);  // 1920 / 20
        assert_eq!(cell.coordinates.height, 216); // 1080 / 5
    }
}
