#[cfg(not(target_os = "windows"))]
pub fn copy_report(_: &str) -> Result<(), String> {
    Err("Clipboard is available in the Windows build".into())
}
#[cfg(target_os = "windows")]
pub fn copy_report(text: &str) -> Result<(), String> {
    use std::ffi::c_void;
    #[link(name = "user32")]
    unsafe extern "system" {
        fn OpenClipboard(owner: *mut c_void) -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(format: u32, memory: *mut c_void) -> *mut c_void;
        fn CloseClipboard() -> i32;
        fn GetActiveWindow() -> *mut c_void;
        fn CreateWindowExW(
            ex: u32,
            class: *const u16,
            title: *const u16,
            style: u32,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            parent: *mut c_void,
            menu: *mut c_void,
            instance: *mut c_void,
            param: *mut c_void,
        ) -> *mut c_void;
        fn DestroyWindow(window: *mut c_void) -> i32;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GlobalAlloc(flags: u32, bytes: usize) -> *mut c_void;
        fn GlobalLock(memory: *mut c_void) -> *mut c_void;
        fn GlobalUnlock(memory: *mut c_void) -> i32;
        fn GlobalFree(memory: *mut c_void) -> *mut c_void;
    }
    let data: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    // Allocate first; clipboard ownership transfers only after SetClipboardData succeeds.
    unsafe {
        let memory = GlobalAlloc(2, data.len() * 2);
        if memory.is_null() {
            return Err("allocation failed".into());
        }
        let pointer = GlobalLock(memory);
        if pointer.is_null() {
            GlobalFree(memory);
            return Err("memory lock failed".into());
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), pointer.cast::<u16>(), data.len());
        GlobalUnlock(memory);
        let active = GetActiveWindow();
        // The standalone reporter has no active AE window. Own a hidden system
        // STATIC window so EmptyClipboard never leaves a null clipboard owner.
        let class: Vec<u16> = "STATIC".encode_utf16().chain(Some(0)).collect();
        let owner = if active.is_null() {
            CreateWindowExW(
                0,
                class.as_ptr(),
                std::ptr::null(),
                0,
                0,
                0,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        } else {
            active
        };
        if owner.is_null() || OpenClipboard(owner) == 0 {
            if active.is_null() && !owner.is_null() {
                DestroyWindow(owner);
            }
            GlobalFree(memory);
            return Err("clipboard is busy or owner creation failed".into());
        }
        let ok = EmptyClipboard() != 0 && !SetClipboardData(13, memory).is_null();
        CloseClipboard();
        if active.is_null() && !owner.is_null() {
            DestroyWindow(owner);
        }
        if !ok {
            GlobalFree(memory);
            return Err("clipboard write failed".into());
        }
    }
    Ok(())
}
