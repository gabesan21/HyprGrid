mod config;
mod grid;

use config::{get_active_monitor, GridDimensions, HyprGridConfig};
use grid::Grid;

fn main() {
    println!("HyprGrid: Initializing...");
    println!();

    // Load configuration
    let config = match HyprGridConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration:");
            eprintln!("{:#}", e);
            eprintln!();
            eprintln!("The Grid remains offline. End of line.");
            std::process::exit(1);
        }
    };

    // Detect active monitor from Hyprland
    let active_monitor = match get_active_monitor() {
        Ok(monitor) => monitor,
        Err(e) => {
            eprintln!("Error detecting active monitor:");
            eprintln!("{:#}", e);
            eprintln!();
            eprintln!("The Grid remains offline. End of line.");
            std::process::exit(1);
        }
    };

    // Calculate grid dimensions with auto-rotation
    let grid_dims = GridDimensions::calculate(
        config.grid_rows,
        config.grid_cols,
        &active_monitor,
    );

    // Generate the complete grid with all cells and coordinates
    let grid = Grid::new(
        grid_dims.rows,
        grid_dims.cols,
        active_monitor.width,
        active_monitor.height,
    );

    // Display configuration report
    println!("Configuration loaded successfully!");
    println!();
    println!("Active Monitor:");
    println!("  Name:        {}", active_monitor.name);
    println!("  Resolution:  {}x{}", active_monitor.width, active_monitor.height);
    println!("  Orientation: {}", grid_dims.orientation.as_str());
    println!();
    println!("Grid Configuration:");
    println!("  Base (landscape): {}x{} (rows x cols)", config.grid_rows, config.grid_cols);
    println!("  Applied:          {}x{} (rows x cols)", grid_dims.rows, grid_dims.cols);
    println!("  Total cells:      {}", grid_dims.total_cells());

    if grid_dims.orientation.as_str() == "Portrait" {
        println!();
        println!("  Note: Grid dimensions auto-rotated for portrait orientation");
    }

    println!();
    println!("Border Settings:");
    println!("  Enabled:  {}", config.border_enabled);
    println!("  Color:    {}", config.border_color);
    println!("  Width:    {} px", config.border_width);

    println!();
    println!("Grid Calculations:");
    println!("  Cell width:   {} px", active_monitor.width / grid.cols);
    println!("  Cell height:  {} px", active_monitor.height / grid.rows);
    println!("  Total cells:  {}", grid.total_cells());

    // Show a sample of cell labels and their coordinates
    println!();
    println!("Sample Cell Coordinates:");
    if let Some(cell) = grid.get_cell("aa") {
        let (cx, cy) = cell.coordinates.center();
        println!("  Cell 'aa': position ({}, {}), center at ({}, {})",
                 cell.coordinates.x, cell.coordinates.y, cx, cy);
    }
    if let Some(cell) = grid.get_cell("as") {
        let (cx, cy) = cell.coordinates.center();
        println!("  Cell 'as': position ({}, {}), center at ({}, {})",
                 cell.coordinates.x, cell.coordinates.y, cx, cy);
    }
    if let Some(cell) = grid.get_cell("ad") {
        let (cx, cy) = cell.coordinates.center();
        println!("  Cell 'ad': position ({}, {}), center at ({}, {})",
                 cell.coordinates.x, cell.coordinates.y, cx, cy);
    }

    println!();
    println!("The Grid is online. End of line.");
}
