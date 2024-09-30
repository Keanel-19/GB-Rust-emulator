#![allow(dead_code, non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use winsafe::{prelude::*, BITMAP, BITMAPINFOHEADER, HBITMAP, HDC, HFILEMAP};
use winsafe::{co, SysResult, BITMAPINFO, HGLOBAL, RGBQUAD};
use winsafe::guard::{DeleteObjectGuard, GlobalFreeGuard};

pub trait ExtendRgbQuad {
    fn new(r: u8, g: u8, b: u8) -> Self;
}

impl ExtendRgbQuad for RGBQUAD {
    fn new(r: u8, g: u8, b: u8) -> Self {
        let mut quad: RGBQUAD = RGBQUAD::default();
        quad.rgbRed = r; quad.rgbGreen = g; quad.rgbBlue = b;
        quad
    }
}

//----------------------------------------------------

pub trait ExtendBitmapinfo {
    #[must_use]
    fn new(header: BITMAPINFOHEADER, colors: &[RGBQUAD]) -> SysResult<BitmapinfoGuard>;
}

impl ExtendBitmapinfo for BITMAPINFO {
    /// Returns a dynamically allocated
    /// [`LogpaletteGuard`](crate::guard::LogpaletteGuard).
    #[must_use]
    fn new(
        header: BITMAPINFOHEADER,
        colors: &[RGBQUAD],
    ) -> SysResult<BitmapinfoGuard>
    {
        BitmapinfoGuard::new(header, colors)
    }
}

//-----------------------------------------------------------------------------------

/// RAII implementation for [`BITMAPINFO`](winsafe::BITMAPINFO) which manages the
/// allocated memory.
pub struct BitmapinfoGuard {
    ptr: GlobalFreeGuard,
    colors_length: usize,
}

impl Deref for BitmapinfoGuard {
    type Target = BITMAPINFO;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr.ptr() as *const _) }
    }
}

impl DerefMut for BitmapinfoGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr.ptr() as *mut _) }
    }
}

impl BitmapinfoGuard {
    #[must_use]
    pub(in crate::gdi_ext) fn new(
        header: BITMAPINFOHEADER,
        colors: &[RGBQUAD],
    ) -> SysResult<Self>
    {
        let sz = std::mem::size_of::<BITMAPINFO>() // size in bytes of the allocated struct
            + ((colors.len() - 1).max(0) * std::mem::size_of::<RGBQUAD>());
            //Quand modifier winsafe : delete ".max(0)" et rendre ".bmiColors" pub(Crate)
        let mut new_self = Self {
            ptr: HGLOBAL::GlobalAlloc(
                Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
                sz,
            )?,
            colors_length: colors.len()
        };
        new_self.bmiHeader = header;
        colors.iter()
            .zip(new_self.bmiColors_mut())
            .for_each(|(src, dest)| *dest = *src); // copy all PALETTEENTRY into struct room
        Ok(new_self)
    }

    /// Returns a constant slice over the `bimColors` entries.
    #[must_use]
    pub fn bmiColors(&self) -> &[RGBQUAD] {
        unsafe {
            std::slice::from_raw_parts(
                self.bmiColors.as_ptr(),
                self.colors_length
            )
        }
    }

    /// Returns a mutable slice over the `bimColors` entries.
    #[must_use]
    pub fn bmiColors_mut(&mut self) -> &mut [RGBQUAD] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.bmiColors.as_mut_ptr(),
                self.colors_length
            )
        }
    }
}

//-----------------------------------------------------------------------------------

mod ffi {
    use winsafe::{prelude::Handle, GetLastError, SysResult};

    type BOOL = i32;
    type COMPTR = *mut std::ffi::c_void;
    type HANDLE = *mut std::ffi::c_void;
    type HRES = u32; // originally declared as i32
    type PCSTR = *const u16;
    type PCVOID = *const std::ffi::c_void;
    type PFUNC = *const std::ffi::c_void;
    type PSTR = *mut u16;
    type PVOID = *mut std::ffi::c_void;

    #[link(name = "gdi32")]
    extern "system" {
        pub fn CreateDIBSection( _: HANDLE, _: PCVOID, _: u32, _: PVOID, _: HANDLE, _: u32 ) -> HANDLE;
    }

    pub(super) fn ptr_to_sysresult(ptr: HANDLE) -> SysResult<HANDLE> {
        if ptr.is_null() {
            Err(GetLastError())
        } else {
            Ok(ptr)
        }
    }
    
    pub(super) fn ptr_to_sysresult_handle<H>(ptr: HANDLE) -> SysResult<H>
        where H: Handle,
    {
        ptr_to_sysresult(ptr)
            .map(|ptr| unsafe { Handle::from_ptr(ptr) })
    }

}

impl gdi_ext_Hdc for HDC {}

#[must_use]
pub trait gdi_ext_Hdc: gdi_Hdc {

    /// [`CreateDIBSection`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createdibsection)
	/// function.
    fn CreateDIBSection(&self,
        bmi: &BITMAPINFO,
        usage: co::DIB,
        bits:  &mut Option<&mut [u8]>,
        section: &HFILEMAP,
        section_offset: u32
    ) -> SysResult<DeleteObjectGuard<HBITMAP>>
    {
        let mut _bits: *mut u8 = std::ptr::null_mut();
        unsafe{
            ffi::ptr_to_sysresult_handle(
                ffi::CreateDIBSection(
                    self.ptr(),
                    bmi as *const _ as _,
                    usage.raw(),
                    &mut _bits as *mut _ as _,
                    section.ptr(),
                    section_offset
                )
            ).map(|h:HBITMAP| {
                let mut bitmap: BITMAP = Default::default();
                let _ = h.GetObject(&mut bitmap).map( |_|
                    *bits = Some(std::slice::from_raw_parts_mut(
                        _bits,
                        (bitmap.bmWidthBytes * bitmap.bmHeight) as usize,
                    ))
                ).map_err(|_| *bits = None);
                DeleteObjectGuard::new(h)
            })
        }.map_err(|e| {*bits = None; e})
    }
}
