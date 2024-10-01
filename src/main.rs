mod gdi_ext;
mod window;

use std::{ops::Deref, sync::mpsc::channel, thread, time::Duration};

use co::DIB;
use window::create_window;
use winsafe::{*, prelude::*};
use gdi_ext::*;

const SCREEN_SIZE: SIZE = SIZE::new(160,144);
const DEFAULT_MULT: i32 = 6;

fn main() -> AnyResult<()>{

    let hdc = HDC::CreateCompatibleDC(&HDC::NULL)?;

    let bitmap_info = BITMAPINFO::new(
        {
            let mut h = BITMAPINFOHEADER::default();
            h.biHeight=-SCREEN_SIZE.cy;
            h.biWidth=SCREEN_SIZE.cx;
            h.biPlanes=1;
            h.biBitCount=4;
            h.biCompression=co::BI::RGB;
            h.biClrUsed=4;
            h
        },
        &[RGBQUAD::new(255, 0, 0), RGBQUAD::new(0, 255, 0), RGBQUAD::new(0, 0, 255), RGBQUAD::new(255, 255, 255)]
    )?;

    let mut bitmap_data = None;

    let bitmap = hdc.CreateDIBSection(&bitmap_info, DIB::RGB_COLORS, &mut bitmap_data, &HFILEMAP::NULL, 0)?;

    let (sender, is_window_running) = channel() ;

    let w = create_window(SCREEN_SIZE, DEFAULT_MULT, unsafe {hdc.raw_copy()}, sender);
    let w_clone = w.clone();
    let window_process = thread::spawn(move || {if let Err(e) = w_clone.run_main(None) {
        HWND::NULL.MessageBox(
            &e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
    }});

    if let Err(_) = is_window_running.recv() {
        return Ok(());
    }

    let _oldbitmap = hdc.SelectObject(bitmap.deref())?;

    if let Some(bytes) = bitmap_data {
        let mut n = 0;

        loop {
            let lwidth = bytes.len() / SCREEN_SIZE.cy as usize;
            for i in 0..SCREEN_SIZE.cy as usize {
                let mut b: u8 = 0;
                for j in 0..SCREEN_SIZE.cx as usize {
                    b += ((i+j).wrapping_sub(n) % 4) as u8;
                    if j % 2 == 0 {
                        b <<= 4;
                    } else {
                        bytes[i*lwidth + j/2] = b;
                        b = 0;
                    }
                }
            }

            if is_window_running.try_recv() != Err(std::sync::mpsc::TryRecvError::Empty) {break};

            w.hwnd().InvalidateRgn(&HRGN::NULL, false);
            w.hwnd().UpdateWindow()?;
            n += 1;
            thread::sleep(Duration::from_millis(17));
        }
    }
    window_process.join().expect("Window error");
    Ok(())
}
