// Configuration module for HyprGrid
// Handles parsing and validation of hg_config.conf and runtime monitor detection

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Represents monitor information detected at runtime from Hyprland
#[derive(Debug, Clone, Deserialize)]
pub struct MonitorInfo {
    /// Monitor name as reported by Hyprland (e.g., "DP-1", "HDMI-A-1")
    pub name: String,
    /// Horizontal resolution in pixels
    pub width: u32,
    /// Vertical resolution in pixels
    pub height: u32,
    /// Whether this monitor is currently focused
    pub focused: bool,
}

/// Monitor orientation detected from dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl Orientation {
    /// Detect orientation from monitor dimensions
    pub fn from_dimensions(width: u32, height: u32) -> Self {
        if width > height {
            Orientation::Landscape
        } else if width < height {
            Orientation::Portrait
        } else {
            Orientation::Square
        }
    }

    /// Get a human-readable string representation
    pub fn as_str(&self) -> &str {
        match self {
            Orientation::Landscape => "Landscape",
            Orientation::Portrait => "Portrait",
            Orientation::Square => "Square",
        }
    }
}

/// Grid dimensions after applying orientation-based rotation
#[derive(Debug, Clone)]
pub struct GridDimensions {
    /// Number of rows in the grid
    pub rows: u32,
    /// Number of columns in the grid
    pub cols: u32,
    /// Detected monitor orientation
    pub orientation: Orientation,
}

impl GridDimensions {
    /// Calculate grid dimensions based on monitor orientation
    ///
    /// # Arguments
    /// - `config_rows`: Configured rows from config file (for landscape)
    /// - `config_cols`: Configured columns from config file (for landscape)
    /// - `monitor`: The monitor to calculate dimensions for
    ///
    /// # Logic
    /// - **Landscape** (width > height): Use configured rows and cols as-is
    /// - **Portrait** (width < height): Swap rows and cols
    /// - **Square** (width == height): Use configured rows and cols as-is
    ///
    /// This ensures grid cells remain roughly square regardless of orientation.
    pub fn calculate(config_rows: u32, config_cols: u32, monitor: &MonitorInfo) -> Self {
        let orientation = Orientation::from_dimensions(monitor.width, monitor.height);

        let (rows, cols) = match orientation {
            Orientation::Landscape | Orientation::Square => (config_rows, config_cols),
            Orientation::Portrait => (config_cols, config_rows), // Swap for portrait
        };

        GridDimensions {
            rows,
            cols,
            orientation,
        }
    }

    /// Get the total number of grid cells
    pub fn total_cells(&self) -> u32 {
        self.rows * self.cols
    }
}

/// Main configuration structure for HyprGrid
#[derive(Debug, Deserialize)]
pub struct HyprGridConfig {
    /// Number of rows in the grid overlay (for landscape orientation)
    pub grid_rows: u32,
    /// Number of columns in the grid overlay (for landscape orientation)
    pub grid_cols: u32,
}

impl HyprGridConfig {
    /// Load and parse the HyprGrid configuration file
    ///
    /// # Configuration File Location
    /// The configuration file should be at: `~/.config/hypr/hg_config.conf`
    ///
    /// # Returns
    /// - `Ok(HyprGridConfig)` if the file is successfully loaded and validated
    /// - `Err` with detailed error message if loading or validation fails
    ///
    /// # Example
    /// ```no_run
    /// use hyprgrid::config::HyprGridConfig;
    ///
    /// let config = HyprGridConfig::load()?;
    /// println!("Grid: {}x{}", config.grid_rows, config.grid_cols);
    /// ```
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        // Read the configuration file
        let config_content = fs::read_to_string(&config_path)
            .with_context(|| {
                format!(
                    "Failed to read configuration file at: {}\n\
                     Please ensure the file exists and is readable.\n\
                     Expected location: ~/.config/hypr/hg_config.conf",
                    config_path.display()
                )
            })?;

        // Parse TOML
        let config: HyprGridConfig = toml::from_str(&config_content)
            .with_context(|| {
                format!(
                    "Failed to parse configuration file at: {}\n\
                     Please check that the TOML syntax is valid.\n\
                     Example format:\n\
                     grid_rows = 10\n\
                     grid_cols = 20",
                    config_path.display()
                )
            })?;

        // Validate the configuration
        config.validate()?;

        Ok(config)
    }

    /// Get the full path to the configuration file
    fn get_config_path() -> Result<PathBuf> {
        let home_dir = std::env::var("HOME")
            .context("HOME environment variable is not set")?;

        let config_path = PathBuf::from(home_dir)
            .join(".config")
            .join("hypr")
            .join("hg_config.conf");

        Ok(config_path)
    }

    /// Validate the configuration values
    ///
    /// Ensures that:
    /// - Grid dimensions are within reasonable bounds (2-50)
    fn validate(&self) -> Result<()> {
        // Validate grid dimensions
        if self.grid_rows < 2 || self.grid_rows > 50 {
            return Err(anyhow!(
                "Invalid grid_rows: {}. Must be between 2 and 50.\n\
                 A reasonable value would be between 5 and 20.",
                self.grid_rows
            ));
        }

        if self.grid_cols < 2 || self.grid_cols > 50 {
            return Err(anyhow!(
                "Invalid grid_cols: {}. Must be between 2 and 50.\n\
                 A reasonable value would be between 10 and 30.",
                self.grid_cols
            ));
        }

        Ok(())
    }
}

/// Get the currently focused monitor from Hyprland
///
/// This function runs `hyprctl monitors -j` to query Hyprland for all monitors
/// and returns the one that is currently focused.
///
/// # Returns
/// - `Ok(MonitorInfo)` with the focused monitor information
/// - `Err` if hyprctl fails, JSON parsing fails, or no focused monitor is found
///
/// # Example
/// ```no_run
/// use hyprgrid::config::get_active_monitor;
///
/// let monitor = get_active_monitor()?;
/// println!("Active monitor: {} ({}x{})", monitor.name, monitor.width, monitor.height);
/// ```
pub fn get_active_monitor() -> Result<MonitorInfo> {
    // Run hyprctl to get monitor information
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .context(
            "Failed to execute 'hyprctl monitors -j'.\n\
             Please ensure you are running Hyprland and hyprctl is installed."
        )?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "hyprctl command failed with status: {}\n\
             stderr: {}",
            output.status,
            stderr
        ));
    }

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let monitors: Vec<MonitorInfo> = serde_json::from_str(&stdout)
        .with_context(|| {
            format!(
                "Failed to parse JSON output from hyprctl.\n\
                 Output was: {}",
                stdout
            )
        })?;

    // Find the focused monitor
    monitors
        .into_iter()
        .find(|m| m.focused)
        .ok_or_else(|| {
            anyhow!(
                "No focused monitor found.\n\
                 This might indicate an issue with your Hyprland setup."
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_detection() {
        assert_eq!(Orientation::from_dimensions(1920, 1080), Orientation::Landscape);
        assert_eq!(Orientation::from_dimensions(1080, 1920), Orientation::Portrait);
        assert_eq!(Orientation::from_dimensions(1080, 1080), Orientation::Square);
    }

    #[test]
    fn test_grid_dimensions_landscape() {
        let monitor = MonitorInfo {
            name: "DP-1".to_string(),
            width: 2560,
            height: 1440,
            focused: true,
        };

        let grid = GridDimensions::calculate(10, 20, &monitor);
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 20);
        assert_eq!(grid.orientation, Orientation::Landscape);
        assert_eq!(grid.total_cells(), 200);
    }

    #[test]
    fn test_grid_dimensions_portrait() {
        let monitor = MonitorInfo {
            name: "DP-1".to_string(),
            width: 1080,
            height: 1920,
            focused: true,
        };

        let grid = GridDimensions::calculate(10, 20, &monitor);
        // Should swap: rows=20, cols=10
        assert_eq!(grid.rows, 20);
        assert_eq!(grid.cols, 10);
        assert_eq!(grid.orientation, Orientation::Portrait);
        assert_eq!(grid.total_cells(), 200);
    }

    #[test]
    fn test_grid_dimensions_square() {
        let monitor = MonitorInfo {
            name: "DP-1".to_string(),
            width: 1920,
            height: 1920,
            focused: true,
        };

        let grid = GridDimensions::calculate(10, 20, &monitor);
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 20);
        assert_eq!(grid.orientation, Orientation::Square);
    }
}
