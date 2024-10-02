use std::sync::mpsc::Sender;

use winsafe::{co::ROP, gui::{self, WindowMain}, prelude::*, HDC, POINT, SIZE};

pub fn create_window(src_size: SIZE, default_mult: i32, src_hdc: HDC, is_running: Sender<bool>) -> WindowMain {
    let window_main: WindowMain = WindowMain::new(gui::WindowMainOpts{
        title: "EmuGB".to_owned(),
        size: ((src_size.cx * default_mult).try_into().unwrap(), (src_size.cy * default_mult).try_into().unwrap()),
        process_dlg_msgs: false,
        ..Default::default()
    });

    let w = window_main.clone();
    window_main.on().wm_paint(move || {
        let hdc = w.hwnd().BeginPaint()?;
        let paint = hdc.paintstruct();
        hdc.StretchBlt(
            POINT { x: paint.rcPaint.left,y: paint.rcPaint.top }, 
            SIZE { cx: paint.rcPaint.right-paint.rcPaint.left, cy: paint.rcPaint.bottom-paint.rcPaint.top }, 
            &src_hdc, 
            POINT { x: 0, y: 0 }, 
            src_size,
            ROP::SRCCOPY)?;
        Ok(())
        }
    );

    let is_running_clone = is_running.clone();
    window_main.on().wm_create(move |_| {
        is_running_clone.send(true)?;
        Ok(0)
    });

    let w = window_main.clone();
    let is_running_clone = is_running.clone();
    window_main.on().wm_close(move || {
        is_running_clone.send(false)?;
        Ok(w.hwnd().DestroyWindow()?)
    });

    window_main.on().wm_erase_bkgnd(|_| {Ok(1)});

    window_main
}
