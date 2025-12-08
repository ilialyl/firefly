#[cfg(target_os = "windows")]
pub mod dpi {
    use windows_sys::Win32::Foundation::E_FAIL;
    use windows_sys::Win32::UI::HiDpi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};

    // Otherwise file dialog on Windows will be blurry
    pub fn enable_dpi_awareness() {
        unsafe {
            let result = SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
            if result != 0 && result != E_FAIL {
                eprintln!("Failed to set DPI awareness, error code: {}", result);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub mod dpi {
    pub fn enable_dpi_awareness() {}
}
