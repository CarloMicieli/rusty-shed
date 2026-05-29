//! Viewport management module for cross-platform resolution-aware window sizing.
//!
//! This module provides functionality to calculate and set appropriate window dimensions
//! based on the physical screen width. It supports Desktop and Android platforms with
//! proper guards for mobile-incompatible APIs.

use tauri::{Runtime, WebviewWindow};

#[cfg(not(mobile))]
use tauri::LogicalSize;

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
trait ViewportWindow {
    fn current_monitor_width(&self) -> Result<u32, Box<dyn std::error::Error>>;
    fn set_min_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>>;
    fn set_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(not(mobile))]
impl<R: Runtime> ViewportWindow for WebviewWindow<R> {
    fn current_monitor_width(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let monitor = self
            .current_monitor()?
            .ok_or("Failed to get current monitor")?;
        Ok(monitor.size().width)
    }

    fn set_min_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.set_min_size(Some(LogicalSize::new(width, height)))?;
        Ok(())
    }

    fn set_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.set_size(LogicalSize::new(width, height))?;
        Ok(())
    }
}

#[cfg(not(mobile))]
fn setup_viewport_for_window(
    window: &impl ViewportWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    let physical_width = window.current_monitor_width()?;

    tracing::info!("Detected monitor with physical width: {}px", physical_width);

    let (width, height) = calculate_min_size(physical_width);

    tracing::info!(
        "Setting window dimensions to {}×{} (logical)",
        width,
        height
    );

    window.set_min_size(width, height)?;
    window.set_size(width, height)?;

    Ok(())
}

#[cfg(not(mobile))]
pub fn setup_viewport<R: Runtime>(
    window: &WebviewWindow<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    setup_viewport_for_window(window)?;

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
    tracing::info!("Viewport setup skipped on mobile platform");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[cfg(not(mobile))]
    use std::cell::RefCell;

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

    #[cfg(not(mobile))]
    #[derive(Default)]
    struct FakeWindow {
        monitor_width: Option<u32>,
        min_size: RefCell<Option<(f64, f64)>>,
        size: RefCell<Option<(f64, f64)>>,
    }

    #[cfg(not(mobile))]
    impl ViewportWindow for FakeWindow {
        fn current_monitor_width(&self) -> Result<u32, Box<dyn std::error::Error>> {
            self.monitor_width
                .ok_or_else(|| "Failed to get current monitor".into())
        }

        fn set_min_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>> {
            *self.min_size.borrow_mut() = Some((width, height));
            Ok(())
        }

        fn set_size(&self, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>> {
            *self.size.borrow_mut() = Some((width, height));
            Ok(())
        }
    }

    #[cfg(not(mobile))]
    #[test]
    fn test_setup_viewport_for_window_sets_expected_dimensions() {
        let window = FakeWindow {
            monitor_width: Some(2560),
            ..Default::default()
        };

        setup_viewport_for_window(&window).expect("setup should succeed");

        assert_eq!(*window.min_size.borrow(), Some((1280.0, 800.0)));
        assert_eq!(*window.size.borrow(), Some((1280.0, 800.0)));
    }

    #[cfg(not(mobile))]
    #[test]
    fn test_setup_viewport_for_window_fails_without_monitor() {
        let window = FakeWindow::default();

        let result = setup_viewport_for_window(&window);

        assert!(result.is_err());
    }
}
