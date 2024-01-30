#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::{
    core::{s, Result},
    Win32::{
        Foundation::{LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            CreateWindowExA, SendMessageA, SC_MONITORPOWER, WINDOW_EX_STYLE, WINDOW_STYLE,
            WM_SYSCOMMAND,
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        let handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("Button"),
            s!(""),
            WINDOW_STYLE::default(),
            0,
            0,
            0,
            0,
            None,
            None,
            None,
            None,
        );

        SendMessageA(
            handle,
            WM_SYSCOMMAND,
            WPARAM(SC_MONITORPOWER as usize),
            LPARAM(2), /* MONITOR_OFF */
        );
    }

    Ok(())
}
