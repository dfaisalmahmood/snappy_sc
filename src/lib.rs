#[cfg(target_os = "windows")]
pub mod windows;

// #[cfg(target_os="macos")]
// pub mod macos;

pub enum OutputFormat {
    Png,
    Jpeg,
    WebP,
}

pub struct CaptureRegion {
    pub start_x: u32,
    pub start_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

pub struct ScreenshotOptions {
    pub display_id: u32,
    pub region: Option<CaptureRegion>,
    pub output_format: OutputFormat,
    // pub with_alpha: bool,
}

pub fn take_screenshot(options: &ScreenshotOptions) -> Result<Vec<u8>, &'static str> {
    #[cfg(target_os = "windows")]
    return windows::take_screenshot(options);

    // #[cfg(target_os="macos")]
    // return macos::take_screenshot(options);
}

pub fn get_focused_display_info() -> Option<(u32, u32, u32)> {
    #[cfg(target_os = "windows")]
    return windows::info::get_focused_display_info();
}
