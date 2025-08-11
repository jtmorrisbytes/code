
impl<T> From<T> for WideString
where
    T: AsRef<OsStr>,
{
    fn from(value: T) -> Self {
        WideString::from_os_str_like(value)
    }
}
impl WidePtr<'_> {
    pub fn as_ptr(&self) -> PCWSTR {
        self.ptr
    }
}
impl WideString {
    pub fn from_str_like<S: AsRef<str>>(s: S) -> Self {
        Self {
            buf: OsStr::new(s.as_ref())
                .encode_wide()
                .chain(std::iter::once(0))
                .collect(),
        }
    }
    pub fn from_os_str_like<S: AsRef<OsStr>>(s: S) -> Self {
        Self {
            buf: s.as_ref().encode_wide().chain(std::iter::once(0)).collect(),
        }
    }
    pub fn as_wideptr<'a>(&'a self) -> WidePtr<'a> {
        WidePtr {
            ptr: PCWSTR(self.buf.as_ptr()),
            _marker: PhantomData,
        }
    }
}

impl Window {
    pub fn set_title(&mut self, title: &str) {
        self.title = WideString::from(title);
        unsafe { SetWindowTextW(self.hwnd, self.title.as_wideptr().as_ptr()) }.unwrap();
    }
    // use windows::core::;
    pub fn initialize<Classname: AsRef<str>, Title: AsRef<str>>(
        class_name: Classname,
        title: Title,
    ) -> Self {
        // create a default instance of 'self'
        let mut self_ = Self {
            hwnd: HWND(std::ptr::null_mut()),
            _hmodule: unsafe { GetModuleHandleW(None) }.unwrap(),
            _window_class: unsafe { zeroed() },
            classname: WideString::from_str_like(class_name),
            title: WideString::from_str_like(title),
        };
        // let mut window_class = 0;

        let r = unsafe {
            GetClassInfoW(
                Some(self_._hmodule.into()),
                self_.classname.as_wideptr().as_ptr(),
                &mut self_._window_class,
            )
        };
        if r.is_err() {
            self_._window_class.lpfnWndProc = Some(__libgame__win32_window_procedure);
            self_._window_class.lpszClassName = self_.classname.as_wideptr().as_ptr();
            self_._window_class.hInstance = self_._hmodule.into();
            self_._window_class.hCursor = unsafe { LoadCursorW(None, IDC_ARROW) }.unwrap();
            _ = unsafe { RegisterClassW(&self_._window_class) }
        }
        let hwnd: HWND = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                self_.classname.as_wideptr().as_ptr(),
                self_.classname.as_wideptr().as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                Some(self_._hmodule.into()),
                None,
            )
            .expect("window should be created")
        };
        self_.hwnd = hwnd;
        let mut buf = vec![0_u16; 255];
        unsafe {
            GetWindowTextW(hwnd, &mut buf);
            println!("{buf:?}");
            dbg!(&self_.title);
            self_
        }
    }
    pub fn show(&self) {
        let _ = unsafe { ShowWindow(self.hwnd, SW_SHOWNORMAL) };
    }

    pub fn run_event_loop(&mut self) {
        let mut msg: MSG = MSG::default();
        for count in 0..usize::MAX {
            let _ = unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) };
            let _ = unsafe { DispatchMessageW(&msg) };

            if count % 100_000 == 0 {
                self.set_title(&format!("{count}"));
            }
        }
    }
}

pub fn poll_window_events() {
    let mut msg: MSG = MSG::default();
    for count in 0..usize::MAX {
        let _ = unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) };
        let _ = unsafe { DispatchMessageW(&msg) };
    }
}

#[test]
pub fn test_win32_window() {
    let mut _window = Window::initialize("Game", "BGameWindow");
    _window.show();
    let _window2 = Window::initialize("Game2", "A second game window");
    _window2.show();
    _window.run_event_loop();
}

#[derive(Debug)]

pub struct WidePtr<'a> {
    ptr: PCWSTR,
    _marker: PhantomData<&'a [u16]>,
}

/// a wrapper for UTF16 encoded data that owns its own buffer
#[derive(Debug)]
pub struct WideString {
    buf: Vec<u16>,
}
pub struct Window {
    classname: WideString,
    title: WideString,
    hwnd: HWND,
    _hmodule: HMODULE,
    _window_class: WNDCLASSW,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn __libgame__win32_window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
}
use std::{ffi::OsStr, marker::PhantomData, mem::zeroed, os::windows::ffi::OsStrExt};
use windows::{
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClassInfoW,
            GetWindowTextW, IDC_ARROW, LoadCursorW, MSG, PM_REMOVE, PeekMessageW, RegisterClassW,
            SW_SHOWNORMAL, SetWindowTextW, ShowWindow, WINDOW_EX_STYLE, WNDCLASSW,
            WS_OVERLAPPEDWINDOW,
        },
    },
    core::PCWSTR,
};
