pub mod info;
use std::{ops::DerefMut, time::Instant};

use crate::{CaptureRegion, OutputFormat, ScreenshotOptions};
use image::{ImageBuffer, Rgba};
use rayon::prelude::*;
use winapi::{
    shared::windef::{DPI_AWARENESS_CONTEXT_SYSTEM_AWARE, HGDIOBJ, RECT},
    um::{
        wingdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits,
            SelectObject, SRCCOPY,
        },
        winuser::{
            GetDesktopWindow, GetSystemMetrics, GetWindowDC, SetThreadDpiAwarenessContext,
            SM_CXSCREEN, SM_CYSCREEN,
        },
    },
};

fn get_display_rect_by_id(display_id: i32) -> Option<RECT> {
    use winapi::{
        shared::{
            minwindef::LPARAM,
            windef::{HDC, HMONITOR},
        },
        um::winuser::EnumDisplayMonitors,
    };

    struct CallbackData {
        target_id: i32,
        current_id: i32,
        rect: Option<RECT>,
    }

    unsafe extern "system" fn monitor_enum_proc(
        _: HMONITOR,
        _: HDC,
        rect: *mut RECT,
        user_data: LPARAM,
    ) -> i32 {
        let user_data = &mut *(user_data as *mut CallbackData);

        if user_data.current_id == user_data.target_id {
            user_data.rect = Some(*rect);
            0 // Stop enumerating
        } else {
            user_data.current_id += 1;
            1 // Continue enumerating
        }
    }

    let mut data = CallbackData {
        target_id: display_id,
        current_id: 0,
        rect: None,
    };
    let user_data_ptr = &mut data as *mut _ as LPARAM;

    unsafe {
        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(monitor_enum_proc),
            user_data_ptr,
        );
    }

    data.rect
}

pub fn take_screenshot(options: &ScreenshotOptions) -> Result<Vec<u8>, &'static str> {
    let start = Instant::now();
    unsafe {
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);
    }
    let hwnd = unsafe { GetDesktopWindow() };
    let hdc = unsafe { GetWindowDC(hwnd) };

    let rect =
        get_display_rect_by_id(options.display_id as i32).ok_or("Failed to get display rect")?;

    // unsafe { GetWindowRect(hwnd, &mut rect) };

    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) as u32 };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) as u32 };

    let default_region = CaptureRegion {
        start_x: rect.left as u32,
        start_y: rect.top as u32,
        end_x: screen_width,
        end_y: screen_height,
    };
    let region = options.region.as_ref().unwrap_or(&default_region);

    let width = region.end_x - region.start_x;
    let height = region.end_y - region.start_y;

    let mem_dc = unsafe { CreateCompatibleDC(hdc) };
    let bitmap = unsafe { CreateCompatibleBitmap(hdc, width as i32, height as i32) };
    let old_bitmap = unsafe { SelectObject(mem_dc, bitmap as HGDIOBJ) };

    unsafe {
        BitBlt(
            mem_dc,
            0,
            0,
            width as i32,
            height as i32,
            hdc,
            region.start_x as i32,
            region.start_y as i32,
            SRCCOPY,
        )
    };

    let bits_per_pixel = 32;
    let mut bitmap_data: Vec<u8> = vec![0; (width * height * (bits_per_pixel / 8)) as usize];

    let bitmap_info = winapi::um::wingdi::BITMAPINFO {
        bmiHeader: winapi::um::wingdi::BITMAPINFOHEADER {
            biSize: std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: bits_per_pixel as u16,
            biCompression: winapi::um::wingdi::BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [winapi::um::wingdi::RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }; 1],
    };

    unsafe {
        GetDIBits(
            mem_dc,
            bitmap,
            0,
            height,
            bitmap_data.as_mut_ptr() as _,
            &bitmap_info as *const _ as *mut _,
            winapi::um::wingdi::DIB_RGB_COLORS,
        )
    };

    unsafe { SelectObject(mem_dc, old_bitmap as HGDIOBJ) };
    unsafe { DeleteDC(mem_dc) };
    unsafe { DeleteObject(bitmap as HGDIOBJ) };
    unsafe { winapi::um::winuser::ReleaseDC(hwnd, hdc) };

    let mut image_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, bitmap_data)
        .ok_or("Failed to create image buffer")?;

    // Swap red and blue channels using rayon for parallel processing
    image_buffer.par_chunks_mut(4).for_each(|pixel| {
        let r = pixel[0];
        let b = pixel[2];
        pixel[0] = b;
        pixel[2] = r;
    });

    // // Swap red and blue channels
    // for y in 0..height {
    //     for x in 0..width {
    //         let pixel = image_buffer.get_pixel_mut(x, y);
    //         let r = pixel[0];
    //         let b = pixel[2];
    //         pixel[0] = b;
    //         pixel[2] = r;
    //     }
    // }

    let mut output = Vec::new();
    match options.output_format {
        OutputFormat::Png => image::codecs::png::PngEncoder::new(&mut output)
            .encode(
                &*image_buffer.as_flat_samples().samples,
                width,
                height,
                image::ColorType::Rgba8,
            )
            .map_err(|_| "Failed to encode PNG")?,
        OutputFormat::Jpeg => image::codecs::jpeg::JpegEncoder::new(&mut output)
            .encode(
                &*image_buffer.as_flat_samples().samples,
                width,
                height,
                image::ColorType::Rgba8,
            )
            .map_err(|_| "Failed to encode JPEG")?,
        OutputFormat::WebP => {
            output = webp::Encoder::new(
                &*image_buffer.as_flat_samples().samples,
                webp::PixelLayout::Rgba,
                width,
                height,
            )
            .encode_lossless()
            .deref_mut()
            .to_vec();
        }
    }

    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("TIME TAKEN: {:?}", duration);
    Ok(output)
}
