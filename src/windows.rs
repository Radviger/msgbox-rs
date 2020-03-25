use icon::IconType;

pub fn create(title: &str, content: &str, icon_type: IconType) {
    use std::iter::once;
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MB_SYSTEMMODAL, MB_DEFAULT_DESKTOP_ONLY,
    };

    let lp_text: Vec<u16> = content.encode_utf16().chain(once(0)).collect();
    let lp_caption: Vec<u16> = title.encode_utf16().chain(once(0)).collect();

    let window_type = match icon_type {
        IconType::Error => MB_OK | MB_ICONERROR | MB_SYSTEMMODAL | MB_DEFAULT_DESKTOP_ONLY,
        IconType::Info => MB_OK | MB_ICONINFORMATION | MB_SYSTEMMODAL | MB_DEFAULT_DESKTOP_ONLY,
        IconType::None => MB_OK | MB_SYSTEMMODAL | MB_DEFAULT_DESKTOP_ONLY,
        IconType::Warning => MB_OK | MB_ICONWARNING | MB_SYSTEMMODAL | MB_DEFAULT_DESKTOP_ONLY,
    };

    unsafe {
        MessageBoxW(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            window_type,
        );
    }
}
