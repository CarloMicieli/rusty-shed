//! Viewport management module for cross-platform resolution-aware window sizing.
//!
//! This module provides functionality to calculate and set appropriate window dimensions
//! based on the physical screen width. It supports Desktop and Android platforms with
//! proper guards for mobile-incompatible APIs.

use tauri::{Runtime, WebviewWindow};

#[cfg(not(mobile))]
use tauri::LogicalSize;

/// Absolute minimum window width (mobile-safe lower bound).
pub const ABSOLUTE_MIN_WIDTH: f64 = 320.0;

/// Absolute minimum window height (mobile-safe lower bound).
pub const ABSOLUTE_MIN_HEIGHT: f64 = 568.0;

/// Preset viewport sizes for developer testing on desktop.
///
/// Each variant represents a well-known viewport dimension that can be applied at
/// runtime via the `set_viewport_preset` Tauri command. On mobile builds the
/// command is a no-op and this enum is still compiled so the types remain coherent.
#[derive(Debug, Clone, PartialEq)]
pub enum ViewportPreset {
    /// Standard desktop window (responsive to monitor DPI via `calculate_initial_size`).
    DesktopDefault,
    /// iPhone-class portrait phone (390 × 844).
    MobilePortrait,
    /// iPhone-class landscape phone (844 × 390).
    MobileLandscape,
    /// iPad-class tablet in portrait (768 × 1024).
    Tablet,
    /// Arbitrary dimensions supplied by the caller.
    Custom(f64, f64),
}

impl ViewportPreset {
    /// Returns the logical `(width, height)` for this preset.
    ///
    /// `physical_width` is only consulted by `DesktopDefault` (to pick the right
    /// DPI-aware desktop initial size). All other variants return fixed dimensions.
    pub fn dimensions(&self, physical_width: u32) -> (f64, f64) {
        match self {
            ViewportPreset::DesktopDefault => calculate_initial_size(physical_width),
            ViewportPreset::MobilePortrait => (390.0, 844.0),
            ViewportPreset::MobileLandscape => (844.0, 390.0),
            ViewportPreset::Tablet => (768.0, 1024.0),
            ViewportPreset::Custom(w, h) => (*w, *h),
        }
    }
}

/// Calculates the initial logical window dimensions for a desktop based on the
/// physical screen width.
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
pub fn calculate_initial_size(physical_width: u32) -> (f64, f64) {
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
/// - **Desktop**: Detects monitor, sets the absolute minimum size to `ABSOLUTE_MIN_WIDTH ×
///   ABSOLUTE_MIN_HEIGHT`, and sets the initial window size to a DPI-aware desktop default.
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

    let (width, height) = calculate_initial_size(physical_width);

    tracing::info!(
        "Setting window initial dimensions to {}×{} (logical)",
        width,
        height
    );

    window.set_min_size(ABSOLUTE_MIN_WIDTH, ABSOLUTE_MIN_HEIGHT)?;
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

/// Applies a `ViewportPreset` to the given window, resizing it immediately.
///
/// The preset dimensions are applied as logical pixels. This function is only
/// compiled on desktop targets; it is a no-op on mobile.
#[cfg(not(mobile))]
pub fn apply_preset<R: Runtime>(
    window: &WebviewWindow<R>,
    preset: &ViewportPreset,
) -> Result<(), Box<dyn std::error::Error>> {
    let physical_width = window
        .current_monitor()?
        .ok_or("Failed to get current monitor")?
        .size()
        .width;

    let (width, height) = preset.dimensions(physical_width);

    tracing::info!(
        "Applying viewport preset {:?} → {}×{} (logical)",
        preset,
        width,
        height
    );

    window.set_size(LogicalSize::new(width, height))?;
    Ok(())
}

/// Tauri command that allows the frontend to switch the desktop window size to a
/// named viewport preset at runtime.
///
/// Accepted preset strings: `"mobile_portrait"`, `"mobile_landscape"`,
/// `"tablet"`, `"desktop"`. Any other string returns an error.
///
/// On mobile builds this command compiles as a no-op that always returns `Ok(())`.
#[tauri::command]
#[specta::specta]
pub fn set_viewport_preset(
    window: WebviewWindow<tauri::Wry>,
    preset: String,
) -> Result<(), String> {
    #[cfg(not(mobile))]
    {
        let vp = match preset.as_str() {
            "mobile_portrait" => ViewportPreset::MobilePortrait,
            "mobile_landscape" => ViewportPreset::MobileLandscape,
            "tablet" => ViewportPreset::Tablet,
            "desktop" => ViewportPreset::DesktopDefault,
            other => return Err(format!("Unknown viewport preset: {other}")),
        };

        apply_preset(&window, &vp).map_err(|e| e.to_string())?;
    }

    #[cfg(mobile)]
    {
        let _ = window;
        let _ = preset;
        tracing::info!("set_viewport_preset is a no-op on mobile");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[cfg(not(mobile))]
    use std::cell::RefCell;

    // ── calculate_initial_size ────────────────────────────────────────────────

    #[test]
    fn test_calculate_initial_size_4k() {
        let (width, height) = calculate_initial_size(3840);
        assert_eq!(width, 1600.0);
        assert_eq!(height, 1000.0);

        // Edge case: just above 3000
        let (width, height) = calculate_initial_size(3001);
        assert_eq!(width, 1600.0);
        assert_eq!(height, 1000.0);
    }

    #[test]
    fn test_calculate_initial_size_qhd() {
        let (width, height) = calculate_initial_size(2560);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);

        // Edge case: just above 2000
        let (width, height) = calculate_initial_size(2001);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);

        // Edge case: at 3000
        let (width, height) = calculate_initial_size(3000);
        assert_eq!(width, 1280.0);
        assert_eq!(height, 800.0);
    }

    #[test]
    fn test_calculate_initial_size_hd() {
        let (width, height) = calculate_initial_size(1920);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);

        // Edge case: at 2000
        let (width, height) = calculate_initial_size(2000);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);

        // Very small display
        let (width, height) = calculate_initial_size(1024);
        assert_eq!(width, 1024.0);
        assert_eq!(height, 768.0);
    }

    // ── ViewportPreset::dimensions ────────────────────────────────────────────

    #[test]
    fn test_preset_mobile_portrait_dimensions() {
        let (w, h) = ViewportPreset::MobilePortrait.dimensions(1920);
        assert_eq!((w, h), (390.0, 844.0));
    }

    #[test]
    fn test_preset_mobile_landscape_dimensions() {
        let (w, h) = ViewportPreset::MobileLandscape.dimensions(1920);
        assert_eq!((w, h), (844.0, 390.0));
    }

    #[test]
    fn test_preset_tablet_dimensions() {
        let (w, h) = ViewportPreset::Tablet.dimensions(1920);
        assert_eq!((w, h), (768.0, 1024.0));
    }

    #[test]
    fn test_preset_desktop_default_defers_to_initial_size() {
        let (w, h) = ViewportPreset::DesktopDefault.dimensions(2560);
        assert_eq!((w, h), calculate_initial_size(2560));
    }

    #[test]
    fn test_preset_custom_dimensions() {
        let (w, h) = ViewportPreset::Custom(1280.0, 720.0).dimensions(1920);
        assert_eq!((w, h), (1280.0, 720.0));
    }

    // ── FakeWindow + setup_viewport_for_window ────────────────────────────────

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
    fn test_setup_viewport_uses_absolute_min_and_initial_size() {
        let window = FakeWindow {
            monitor_width: Some(2560),
            ..Default::default()
        };

        setup_viewport_for_window(&window).expect("setup should succeed");

        // Minimum size must be the absolute mobile-safe lower bound.
        assert_eq!(
            *window.min_size.borrow(),
            Some((ABSOLUTE_MIN_WIDTH, ABSOLUTE_MIN_HEIGHT))
        );
        // Initial size is the DPI-aware desktop default, not the minimum.
        assert_eq!(*window.size.borrow(), Some((1280.0, 800.0)));
    }

    #[cfg(not(mobile))]
    #[test]
    fn test_setup_viewport_min_and_initial_differ_on_hd_monitor() {
        let window = FakeWindow {
            monitor_width: Some(1920),
            ..Default::default()
        };

        setup_viewport_for_window(&window).expect("setup should succeed");

        assert_eq!(
            *window.min_size.borrow(),
            Some((ABSOLUTE_MIN_WIDTH, ABSOLUTE_MIN_HEIGHT))
        );
        assert_eq!(*window.size.borrow(), Some((1024.0, 768.0)));
    }

    #[cfg(not(mobile))]
    #[test]
    fn test_setup_viewport_for_window_fails_without_monitor() {
        let window = FakeWindow::default();

        let result = setup_viewport_for_window(&window);

        assert!(result.is_err());
    }

    // ── apply_preset (FakeWindow) ─────────────────────────────────────────────
    //
    // apply_preset uses the real WebviewWindow API, so we test the preset
    // dimension logic through ViewportPreset::dimensions instead of duplicating
    // the integration concern here.  The setup tests above already verify that
    // set_size is called with the right values when the window impl is faked.

    #[cfg(not(mobile))]
    #[test]
    fn test_fake_window_apply_preset_mobile_portrait() {
        // Simulate what apply_preset does via FakeWindow.
        let window = FakeWindow {
            monitor_width: Some(1920),
            ..Default::default()
        };

        let physical_width = window.current_monitor_width().unwrap();
        let (w, h) = ViewportPreset::MobilePortrait.dimensions(physical_width);
        window.set_size(w, h).unwrap();

        assert_eq!(*window.size.borrow(), Some((390.0, 844.0)));
    }

    #[cfg(not(mobile))]
    #[test]
    fn test_fake_window_apply_preset_tablet() {
        let window = FakeWindow {
            monitor_width: Some(1920),
            ..Default::default()
        };

        let physical_width = window.current_monitor_width().unwrap();
        let (w, h) = ViewportPreset::Tablet.dimensions(physical_width);
        window.set_size(w, h).unwrap();

        assert_eq!(*window.size.borrow(), Some((768.0, 1024.0)));
    }
}
