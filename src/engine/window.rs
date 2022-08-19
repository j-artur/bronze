use std::mem::size_of;
use std::process::exit;
use std::ptr::{null, null_mut};
use winapi::shared::{minwindef::*, windef::*, windowsx::*};
use winapi::um::{libloaderapi::*, wingdi::*, winuser::*};

use super::{rgb, u16str, Color};

static mut KEYS: [bool; 256] = [false; 256];
static mut MOUSE_POS: Point = (0, 0);

pub type Point = (i32, i32);
pub type Size = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

use WindowMode::*;

pub struct Window {
    instance: HINSTANCE,
    handle: HWND,
    size: Size,
    icon: HICON,
    cursor: HCURSOR,
    bg_color: Color,
    title: String,
    style: DWORD,
    mode: WindowMode,
    pos: Point,
    center: Point,
}

impl Window {
    pub fn new() -> Self {
        unsafe {
            Window {
                instance: GetModuleHandleW(null()),
                handle: null_mut(),
                size: (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)),
                icon: LoadIconW(null_mut(), IDI_APPLICATION),
                cursor: LoadCursorW(null_mut(), IDC_ARROW),
                bg_color: rgb!(0, 0, 0),
                title: String::new(),
                style: WS_POPUP | WS_VISIBLE,
                mode: WindowMode::Fullscreen,
                pos: (0, 0),
                center: (0, 0),
            }
        }
    }

    pub fn create(&mut self) -> bool {
        let wndclassname = u16str!("GameWindow");

        let wndclass = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as UINT,
            style: CS_DBLCLKS | CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(win_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: self.instance,
            hIcon: self.icon,
            hCursor: self.cursor,
            hbrBackground: {
                let Color(r, g, b, _) = self.bg_color;
                unsafe { CreateSolidBrush(RGB(r, g, b)) }
            },
            lpszMenuName: null(),
            lpszClassName: wndclassname,
            hIconSm: self.icon,
        };

        if unsafe { RegisterClassExW(&wndclass) } == 0 {
            return false;
        }

        self.handle = unsafe {
            CreateWindowExW(
                0,
                wndclassname,
                u16str!(&self.title),
                self.style,
                self.pos.0,
                self.pos.1,
                self.size.0,
                self.size.1,
                null_mut(),
                null_mut(),
                self.instance,
                null_mut(),
            )
        };

        if self.mode == WindowMode::Windowed {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: self.size.0,
                bottom: self.size.1,
            };

            unsafe {
                AdjustWindowRectEx(
                    &mut rect,
                    GetWindowLongW(self.handle, GWL_STYLE) as DWORD,
                    (GetMenu(self.handle) != null_mut()).into(),
                    GetWindowLongW(self.handle, GWL_EXSTYLE) as DWORD,
                );

                self.pos = (
                    (GetSystemMetrics(SM_CXSCREEN) - rect.right + rect.left) / 2,
                    (GetSystemMetrics(SM_CYSCREEN) - rect.bottom + rect.top) / 2,
                );

                MoveWindow(
                    self.handle,
                    self.pos.0,
                    self.pos.1,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    1,
                );
            };
        }

        self.handle != null_mut()
    }

    pub fn instance(&self) -> HINSTANCE {
        self.instance
    }

    pub fn handle(&self) -> HWND {
        self.handle
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;

        self.center = (size.0 / 2, size.1 / 2);

        self.pos = (
            (unsafe { GetSystemMetrics(SM_CXSCREEN) } - self.size.0) / 2,
            (unsafe { GetSystemMetrics(SM_CYSCREEN) } - self.size.1) / 2,
        );
    }

    pub fn set_icon(&mut self, icon: u16) {
        self.icon = unsafe { LoadIconW(GetModuleHandleW(null()), MAKEINTRESOURCEW(icon)) };
    }

    pub fn set_cursor(&mut self, cursor: u16) {
        self.cursor = unsafe { LoadCursorW(GetModuleHandleW(null()), MAKEINTRESOURCEW(cursor)) };
    }

    pub fn bg(&self) -> Color {
        self.bg_color
    }

    pub fn set_bg(&mut self, color: Color) {
        self.bg_color = color;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: WindowMode) {
        self.mode = mode;
        self.style = match mode {
            Windowed => WS_OVERLAPPED | WS_SYSMENU | WS_VISIBLE,
            Fullscreen => WS_EX_TOPMOST | WS_POPUP | WS_VISIBLE,
        }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn key_down(&self, key: u8) -> bool {
        unsafe { KEYS[key as usize] }
    }

    pub fn key_up(&self, key: u8) -> bool {
        unsafe { !KEYS[key as usize] }
    }

    pub fn mouse(&self) -> Point {
        unsafe { MOUSE_POS }
    }

    pub fn mouse_x(&self) -> i32 {
        unsafe { MOUSE_POS.0 }
    }

    pub fn mouse_y(&self) -> i32 {
        unsafe { MOUSE_POS.1 }
    }

    pub fn hide_cursor(&self, hide: bool) {
        unsafe { ShowCursor(!hide as BOOL) };
    }

    pub fn close(&self) {
        unsafe { PostMessageW(self.handle, WM_DESTROY, 0, 0) };
    }
}

pub unsafe extern "system" fn win_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_MOUSEMOVE => MOUSE_POS = (GET_X_LPARAM(lparam), GET_Y_LPARAM(lparam)),
        WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => KEYS[VK_LBUTTON as usize] = true,
        WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => KEYS[VK_MBUTTON as usize] = true,
        WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => KEYS[VK_RBUTTON as usize] = true,
        WM_LBUTTONUP => KEYS[VK_LBUTTON as usize] = false,
        WM_MBUTTONUP => KEYS[VK_MBUTTON as usize] = false,
        WM_RBUTTONUP => KEYS[VK_RBUTTON as usize] = false,
        WM_KEYDOWN => KEYS[wparam as usize] = true,
        WM_KEYUP => KEYS[wparam as usize] = false,
        WM_DESTROY => {
            PostQuitMessage(0);
            exit(0)
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    };

    0
}
