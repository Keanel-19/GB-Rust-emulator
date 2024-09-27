#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use winsafe::{prelude::*, BITMAPINFOHEADER};
use winsafe::{co, SysResult, BITMAPINFO, HGLOBAL, RGBQUAD};
use winsafe::guard::GlobalFreeGuard;

fn len_bmiColors(hd: &BITMAPINFOHEADER) -> usize {
    if hd.biCompression == co::BI::RGB && hd.biBitCount <= 8 {
        if hd.biClrUsed > 0 {
            hd.biClrUsed as _
        } else {
            1 << hd.biBitCount
        }
    } else {
        1
    }
}

const fn stride(hd: &BITMAPINFOHEADER) -> usize {
    (((hd.biWidth as usize * hd.biBitCount as usize) + 31) & !31) >> 3
}

const fn size(hd: &BITMAPINFOHEADER) -> usize {
    hd.biHeight.abs() as usize * stride(hd)
}

//----------------------------------------------------

pub trait ExtendBitmapinfo {
    #[must_use]
    fn new(header: BITMAPINFOHEADER, colors: &[RGBQUAD]) -> SysResult<BitmapinfoGuard>;
    #[must_use]
    fn bmiColors(&self) -> &[RGBQUAD];
    #[must_use]
    fn bmiColors_mut(&mut self) -> &mut [RGBQUAD];
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

    /// Returns a constant slice over the `palPalEntry` entries.
    #[must_use]
    fn bmiColors(&self) -> &[RGBQUAD] {
        unsafe {
            std::slice::from_raw_parts(
                self.bmiColors.as_ptr(),
                len_bmiColors(&self.bmiHeader),
            )
        }
    }

    /// Returns a mutable slice over the `palPalEntry` entries.
    #[must_use]
    fn bmiColors_mut(&mut self) -> &mut [RGBQUAD] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.bmiColors.as_mut_ptr(),
                len_bmiColors(&self.bmiHeader),
            )
        }
    }
}

//-----------------------------------------------------------------------------------

/// RAII implementation for [`BITMAPINFO`](winsafe::BITMAPINFO) which manages the
/// allocated memory.
pub struct BitmapinfoGuard {
    ptr: GlobalFreeGuard,
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
        assert_eq!(len_bmiColors(&header),colors.len());
        
        let sz = std::mem::size_of::<BITMAPINFO>() // size in bytes of the allocated struct
            - std::mem::size_of::<RGBQUAD>()
            + (colors.len() * std::mem::size_of::<RGBQUAD>());
        let mut new_self = Self {
            ptr: HGLOBAL::GlobalAlloc(
                Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
                sz,
            )?,
        };
        new_self.bmiHeader = header;
        colors.iter()
            .zip(new_self.bmiColors_mut())
            .for_each(|(src, dest)| *dest = *src); // copy all PALETTEENTRY into struct room
        Ok(new_self)
    }
}