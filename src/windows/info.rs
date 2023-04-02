use std::ffi::c_void;
use winapi::{
    shared::{
        minwindef::LPARAM,
        windef::{DPI_AWARENESS_CONTEXT_SYSTEM_AWARE, HDC, HMONITOR, RECT},
    },
    um::winuser::{
        EnumDisplayMonitors, GetForegroundWindow, GetMonitorInfoA, MonitorFromWindow,
        SetThreadDpiAwarenessContext, MONITORINFO,
    },
};

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    display_index: LPARAM,
) -> i32 {
    let display_index = &mut *(display_index as *mut i32);
    *display_index += 1;
    1
}

pub fn get_focused_display_info() -> Option<(i32, u32, u32)> {
    unsafe {
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);
        let foreground_window = GetForegroundWindow();
        if foreground_window.is_null() {
            return None;
        }

        let focused_monitor_handle = MonitorFromWindow(
            foreground_window,
            winapi::um::winuser::MONITOR_DEFAULTTONEAREST,
        );

        let mut display_index: i32 = -1;
        let mut user_data = &mut display_index as *mut _ as LPARAM;

        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(monitor_enum_proc),
            user_data,
        );

        let mut monitor_info: MONITORINFO = std::mem::zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

        if GetMonitorInfoA(focused_monitor_handle, &mut monitor_info as *mut _) == 0 {
            return None;
        }

        let width = (monitor_info.rcMonitor.right - monitor_info.rcMonitor.left) as u32;
        let height = (monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top) as u32;

        Some((display_index, width, height))
    }
}
