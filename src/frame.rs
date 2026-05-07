use crate::error::{check, Error, Result};
use crate::types::{FrameInfo, TextureFormat, TextureOrigin, TransferMode, SyncMode};
use crate::ffi;

use std::ptr;

pub struct Frame {
    raw: *mut ffi::NozzleFrame,
}

impl Frame {
    pub(crate) fn from_raw(raw: *mut ffi::NozzleFrame) -> Self {
        Frame { raw }
    }

    pub fn info(&self) -> Result<FrameInfo> {
        let mut info = ffi::NozzleFrameInfo {
            frame_index: 0,
            timestamp_ns: 0,
            width: 0,
            height: 0,
            format: 0,
            semantic_format: 0,
            transfer_mode: 0,
            sync_mode: 0,
            dropped_frame_count: 0,
        };
        let rc = unsafe { ffi::nozzle_frame_get_info(self.raw, &mut info) };
        check(rc as _)?;
        Ok(FrameInfo {
            frame_index: info.frame_index,
            timestamp_ns: info.timestamp_ns,
            width: info.width,
            height: info.height,
            format: TextureFormat::from_raw(info.format as _),
            semantic_format: TextureFormat::from_raw(info.semantic_format as _),
            transfer_mode: TransferMode::from_raw(info.transfer_mode as _),
            sync_mode: SyncMode::from_raw(info.sync_mode as _),
            dropped_frame_count: info.dropped_frame_count,
        })
    }

    pub fn lock_pixels(&mut self) -> Result<MappedPixels<'_>> {
        self.map_pixels(false)
    }

    pub fn lock_writable_pixels(&mut self) -> Result<MappedPixels<'_>> {
        self.map_pixels(true)
    }

    fn map_pixels(&mut self, writable: bool) -> Result<MappedPixels<'_>> {
        let mut mapped = ffi::NozzleMappedPixels {
            data: ptr::null_mut(),
            row_stride_bytes: 0,
            width: 0,
            height: 0,
            format: 0,
            origin: 0,
        };

        let rc = if writable {
            unsafe { ffi::nozzle_frame_lock_writable_pixels_with_origin(self.raw, 0, &mut mapped) }
        } else {
            unsafe { ffi::nozzle_frame_lock_pixels_with_origin(self.raw, 0, &mut mapped) }
        };
        check(rc as _)?;

        let len = (mapped.height as usize)
            .checked_mul(mapped.row_stride_bytes as usize)
            .ok_or_else(|| Error::with_message(crate::ErrorCode::Unknown, "pixel buffer size overflow"))?;

        let data = unsafe {
            std::slice::from_raw_parts_mut(mapped.data as *mut u8, len)
        };

        Ok(MappedPixels {
            frame: self.raw,
            data,
            row_stride_bytes: mapped.row_stride_bytes as u32,
            width: mapped.width,
            height: mapped.height,
            format: TextureFormat::from_raw(mapped.format as _),
            origin: TextureOrigin::from_raw(mapped.origin as _),
            writable,
        })
    }

    pub fn copy_to_gl_texture(
        &self,
        gl_texture_name: u32,
        gl_target: u32,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Result<()> {
        let rc = unsafe {
            ffi::nozzle_frame_copy_to_gl_texture(
                self.raw,
                gl_texture_name,
                gl_target,
                width,
                height,
                format as _,
            )
        };
        check(rc as _)
    }

    pub(crate) fn into_raw(self) -> *mut ffi::NozzleFrame {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::nozzle_frame_release(self.raw) };
        }
    }
}

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}

pub struct MappedPixels<'a> {
    frame: *mut ffi::NozzleFrame,
    data: &'a mut [u8],
    pub row_stride_bytes: u32,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub origin: TextureOrigin,
    writable: bool,
}

impl<'a> MappedPixels<'a> {
    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    pub fn row(&self, y: u32) -> Option<&[u8]> {
        if y >= self.height {
            return None;
        }
        let start = (y as usize) * (self.row_stride_bytes as usize);
        let end = start + (self.row_stride_bytes as usize);
        Some(&self.data[start..end])
    }

    pub fn row_mut(&mut self, y: u32) -> Option<&mut [u8]> {
        if y >= self.height {
            return None;
        }
        let start = (y as usize) * (self.row_stride_bytes as usize);
        let end = start + (self.row_stride_bytes as usize);
        Some(&mut self.data[start..end])
    }
}

impl<'a> Drop for MappedPixels<'a> {
    fn drop(&mut self) {
        if self.frame.is_null() {
            return;
        }
        if self.writable {
            unsafe { ffi::nozzle_frame_unlock_writable_pixels(self.frame) };
        } else {
            unsafe { ffi::nozzle_frame_unlock_pixels(self.frame) };
        }
    }
}

pub struct WritableFrame {
    inner: Frame,
}

impl WritableFrame {
    pub(crate) fn from_raw(raw: *mut ffi::NozzleFrame) -> Self {
        WritableFrame {
            inner: Frame::from_raw(raw),
        }
    }

    pub fn info(&self) -> Result<FrameInfo> {
        self.inner.info()
    }

    pub fn lock_pixels(&mut self) -> Result<MappedPixels<'_>> {
        self.inner.lock_pixels()
    }

    pub fn lock_writable_pixels(&mut self) -> Result<MappedPixels<'_>> {
        self.inner.lock_writable_pixels()
    }

    pub(crate) fn into_raw(self) -> *mut ffi::NozzleFrame {
        self.inner.into_raw()
    }
}

unsafe impl Send for WritableFrame {}
unsafe impl Sync for WritableFrame {}
