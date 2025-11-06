mod config;
mod grid;

use config::{get_active_monitor, GridDimensions, HyprGridConfig};

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
    println!("The Grid is online. End of line.");
}
