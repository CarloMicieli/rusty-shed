//! Viewport management module for cross-platform resolution-aware window sizing.
//!
//! This module provides functionality to calculate and set appropriate window dimensions
//! based on the physical screen width. It supports Desktop and Android platforms with
//! proper guards for mobile-incompatible APIs.

use tauri::{LogicalSize, Runtime, WebviewWindow};

/// Calculates the minimum logical window dimensions based on physical screen width.
///
/// This pure function determines appropriate window dimensions based on three thresholds:
/// - **4K displays** (> 3000px physical): 1600×1000 (accounts for 2x DPI scaling)
/// - **QHD displays** (> 2000px physical): 1280×800
/// - **HD/Default displays** (≤ 2000px): 1024×768
///
/// # Arguments
///
/// * `physical_width` - The physical width of the screen in pixels
///
/// # Returns
///
/// A tuple of `(width, height)` representing logical dimensions in pixels
pub fn calculate_min_size(physical_width: u32) -> (f64, f64) {
    match physical_width {
        w if w > 3000 => (1600.0, 1000.0), // 4K - larger to account for high DPI
        w if w > 2000 => (1280.0, 800.0),  // QHD
        _ => (1024.0, 768.0),              // HD
    }
}

/// Sets up the viewport for the given window based on the current monitor's dimensions.
///
/// This function detects the physical width of the monitor and applies appropriate
/// minimum and initial window sizes. On Desktop platforms, it will resize the window
/// accordingly. On mobile platforms, window resizing APIs are skipped.
///
/// # Platform-Specific Behavior
///
/// - **Desktop**: Detects monitor, calculates dimensions, and applies window sizing
/// - **Mobile/Android**: Function is available but window sizing operations are no-ops
///
/// # Arguments
///
/// * `window` - A reference to the Tauri `WebviewWindow`
///
/// # Errors
///
/// Returns an error if:
/// - Monitor detection fails (Desktop only)
/// - Window sizing operations fail (Desktop only)
#[cfg(not(mobile))]
pub fn setup_viewport<R: Runtime>(
    window: &WebviewWindow<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the current monitor
    let monitor = window
        .current_monitor()?
        .ok_or("Failed to get current monitor")?;

    // Get physical size
    let physical_size = monitor.size();
    let physical_width = physical_size.width;

    log::info!("Detected monitor with physical width: {}px", physical_width);

    // Calculate appropriate dimensions
    let (width, height) = calculate_min_size(physical_width);

    log::info!(
        "Setting window dimensions to {}×{} (logical)",
        width,
        height
    );

    // Apply minimum size constraints
    let min_size = LogicalSize::new(width, height);
    window.set_min_size(Some(min_size))?;

    // Set initial size to the minimum
    let size = LogicalSize::new(width, height);
    window.set_size(size)?;

    Ok(())
}

/// Mobile-compatible stub for viewport setup.
///
/// On mobile platforms, this function is available but performs no operations,
/// as window resizing APIs are not supported on Android.
///
/// # Arguments
///
/// * `window` - A reference to the Tauri `WebviewWindow`
///
/// # Returns
///
/// Always returns `Ok(())` on mobile platforms
#[cfg(mobile)]
pub fn setup_viewport<R: Runtime>(
    _window: &WebviewWindow<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Viewport setup skipped on mobile platform");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_min_size_4k() {
        // Test 4K threshold (> 3000px)
        let (width, height) = calculate_min_size(3840);
        assert_eq!(width, 1600.0);
        assert_eq!(height, 1000.0);

        // Edge case: just above 3000
        let (width, height) = calculate_min_size(3001);
        assert_eq!(width, 1600.0);
        assert_eq!(height, 1000.0);
    }

    #[test]
    fn test_calculate_min_size_qhd() {
        // Test QHD threshold (> 2000px, <= 3000px)
        let (width, height) = calculate_min_size(2560);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);
        // Edge case: just above 2000
        let (width, height) = calculate_min_size(2001);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);

        // Edge case: at 3000
        let (width, height) = calculate_min_size(3000);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);
    }

    #[test]
    fn test_calculate_min_size_hd() {
        // Test HD/Default threshold (<= 2000px)
        let (width, height) = calculate_min_size(1920);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);

        // Edge case: at 2000
        let (width, height) = calculate_min_size(2000);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);

        // Very small display
        let (width, height) = calculate_min_size(1024);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);
    }
}
