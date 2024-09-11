fn main() {
    windows::build!(
        windows::Win32::Foundation::{HWND, LPARAM, PWSTR, WPARAM},
        windows::Win32::System::LibraryLoader::GetModuleHandleA,
        windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, DispatchMessageA, GetMessageA, RegisterClassA, TranslateMessage, MSG,
            WM_WTSSESSION_CHANGE, WNDCLASSA, WTS_SESSION_LOCK, WTS_SESSION_UNLOCK,
        },
        windows::Win32::System::WtsApi32::WTSRegisterSessionNotification
    );
}
