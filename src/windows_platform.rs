use std::os::raw::c_void;
use windows::core::PCSTR;
use windows::Win32::System::RemoteDesktop::WTSRegisterSessionNotification;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
    System::LibraryLoader::GetModuleHandleA,
    UI::WindowsAndMessaging::{
        DefWindowProcA, DispatchMessageA, GetMessageA, TranslateMessage, MSG,
    },
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    RegisterClassA, CS_HREDRAW, CS_VREDRAW, WNDCLASSA,
};

pub async fn windows_lock_unlock() {
    unsafe {
        let hinstance: *mut c_void = match GetModuleHandleA(None) {
            Ok(handle) => handle.0 as *mut c_void,
            Err(_) => std::ptr::null_mut(),
        };

        let class_name = std::ffi::CString::new("my_window_class").unwrap();

        let wnd_class = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr() as *const u8,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
        };

        RegisterClassA(&wnd_class);

        let hwnd = match CreateWindowExA(
            WS_EX_NOACTIVATE,
            PCSTR(class_name.as_ptr() as *const u8),
            PCSTR(class_name.as_ptr() as *const u8),
            WS_OVERLAPPEDWINDOW & !WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND(std::ptr::null_mut()),
            HMENU(std::ptr::null_mut()),
            HINSTANCE(hinstance as _),
            None,
        ) {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Failed to create window: {:?}", e);
                return;
            }
        };

        if let Err(e) = WTSRegisterSessionNotification(hwnd, 0) {
            eprintln!("Failed to register session notification: {:?}", e);
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, HWND(std::ptr::null_mut()), 0, 0) != false {
            if TranslateMessage(&msg) == false {
                eprintln!("Failed to translate message");
            }
            DispatchMessageA(&msg);
        }

        match DstroyWindow(hwnd) {
            Ok(_) => {
                println!("Window destroyed");
            }
            Err(e) => {
                eprintln!("Failed to destroy window: {:?}", e);
            }
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: *mut c_void,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    let hwnd = HWND(hwnd as _);
    let wparam = WPARAM(wparam);
    let lparam = LPARAM(lparam);
    match msg {
        WM_WTSSESSION_CHANGE => match wparam.0 as u32 {
            WTS_SESSION_LOCK => {
                println!("Session locked");
            }
            WTS_SESSION_UNLOCK => {
                println!("Session unlocked");
            }
            _ => {}
        },
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam).0 as isize,
    }
    0
}
