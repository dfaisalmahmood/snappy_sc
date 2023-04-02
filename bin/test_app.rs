use snappy_sc::{OutputFormat, ScreenshotOptions};
use std::fs::File;
use std::io::Write;

fn main() {
    let options = ScreenshotOptions {
        display_id: 0,
        region: None,
        output_format: OutputFormat::Png,
    };

    let (display_id, width, height) = snappy_sc::get_focused_display_info().unwrap();
    println!("DISPLAY: {}, {}, {}", display_id, width, height);
    match snappy_sc::take_screenshot(&options) {
        Ok(screenshot_data) => {
            let mut file = File::create("screenshot.png").expect("Unable to create file");
            file.write_all(&screenshot_data)
                .expect("Unable to write data to file");
            println!("Screenshot saved as screenshot.png");
        }
        Err(e) => {
            eprintln!("Error capturing screenshot: {}", e);
        }
    }
}
