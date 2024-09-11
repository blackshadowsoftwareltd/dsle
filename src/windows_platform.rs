// extern crate windows_sys as sys;

use sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
use sys::Win32::System::LibraryLoader::GetModuleHandleA;
use sys::Win32::System::Threading::GetCurrentProcessId;
use sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcA, DispatchMessageA, GetMessageA, RegisterClassA, TranslateMessage, MSG,
    WM_WTSSESSION_CHANGE, WNDCLASSA, WTS_SESSION_LOCK, WTS_SESSION_UNLOCK,
};
use windows::Win32::System::WtsApi32::WTSRegisterSessionNotification;
use windows_sys::core::PWSTR;
use windows_sys::Win32::UI::WindowsAndMessaging;

pub async fn windows_lock_unlock() {
    windows::build!(Windows::Win32::System::WtsApi32::WTSRegisterSessionNotification);
    unsafe {
        let hinstance = GetModuleHandleA(std::ptr::null_mut());

        let lpsz_class_name = "MyClass\0".as_ptr();

        let wnd_class = WNDCLASSA {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: lpsz_class_name,
        };

        RegisterClassA(&wnd_class);
        let hwnd: HWND = std::ptr::null_mut();
        WTSRegisterSessionNotification(hwnd, 0);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> isize {
    match msg {
        WM_WTSSESSION_CHANGE => match wparam.0 {
            WTS_SESSION_LOCK => println!("Screen locked"),
            WTS_SESSION_UNLOCK => println!("Screen unlocked"),
            _ => (),
        },
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
    0
}
