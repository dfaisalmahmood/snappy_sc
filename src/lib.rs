#[cfg(target_os = "windows")]
pub mod windows;

// #[cfg(target_os="macos")]
// pub mod macos;

pub enum OutputFormat {
    Png,
    Jpeg,
    WebP,
}

/// Capture region for screenshot
pub struct CaptureRegion {
    pub start_x: u32,
    pub start_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

/// Screenshot options
pub struct ScreenshotOptions {
    /// Display ID for which to take screenshot.
    /// Use `get_focused_display_info` to get ID of focussed display`)
    pub display_id: u32,
    /// Region for which to take screenshot.
    /// Leave empty for entire screen.
    pub region: Option<CaptureRegion>,
    /// The output format for the screenshot.
    pub output_format: OutputFormat,
    // pub with_alpha: bool,
}

/// Take screenshot and returns byte array of specified output format
/// 
/// # Arguments
/// 
/// * `options` - Options to specify region, display, etc
pub fn take_screenshot(options: &ScreenshotOptions) -> Result<Vec<u8>, &'static str> {
    #[cfg(target_os = "windows")]
    return windows::take_screenshot(options);

    // #[cfg(target_os="macos")]
    // return macos::take_screenshot(options);
}

/// Get the display info for the currently focused display.
/// Returns tuple of `(display_id, width, height)`
pub fn get_focused_display_info() -> Option<(u32, u32, u32)> {
    #[cfg(target_os = "windows")]
    return windows::info::get_focused_display_info();
}