use crate::error::{check, Result};
use crate::ffi;
use crate::frame::WritableFrame;
use crate::types::{SenderDesc, SenderInfo};

use std::ffi::CString;
use std::mem;
use std::ptr;

pub struct Sender {
    raw: *mut ffi::NozzleSender,
}

impl Sender {
    pub fn create(desc: &SenderDesc) -> Result<Self> {
        let name = CString::new(&desc.name[..]).map_err(|_| {
            crate::error::Error::with_message(
                crate::ErrorCode::InvalidArgument,
                "sender name contains nul byte",
            )
        })?;
        let app_name = CString::new(&desc.application_name[..]).map_err(|_| {
            crate::error::Error::with_message(
                crate::ErrorCode::InvalidArgument,
                "application name contains nul byte",
            )
        })?;

        let mut c_desc: ffi::NozzleSenderDesc = unsafe { mem::zeroed() };
        c_desc.name = name.as_ptr();
        c_desc.application_name = app_name.as_ptr();
        c_desc.ring_buffer_size = desc.ring_buffer_size;
        c_desc.fallback_flags = desc.fallback_flags;
        c_desc.fallback_flags_valid = if desc.fallback_flags_valid { 1 } else { 0 };

        let mut sender_ptr: *mut ffi::NozzleSender = ptr::null_mut();
        let rc = unsafe { ffi::nozzle_sender_create(&c_desc, &mut sender_ptr) };
        check(rc as _)?;

        Ok(Sender { raw: sender_ptr })
    }

    pub fn acquire_writable_frame(
        &mut self,
        width: u32,
        height: u32,
        format: crate::types::TextureFormat,
    ) -> Result<WritableFrame> {
        let mut frame_ptr: *mut ffi::NozzleFrame = ptr::null_mut();
        let rc = unsafe {
            ffi::nozzle_sender_acquire_writable_frame(
                self.raw,
                width,
                height,
                format as _,
                &mut frame_ptr,
            )
        };
        check(rc as _)?;
        Ok(WritableFrame::from_raw(frame_ptr))
    }

    pub fn commit_frame(&mut self, frame: WritableFrame) -> Result<()> {
        let raw = frame.into_raw();
        let rc = unsafe { ffi::nozzle_sender_commit_frame(self.raw, raw) };
        check(rc as _)
    }

    pub fn publish_gl_texture(
        &mut self,
        gl_texture_name: u32,
        gl_target: u32,
        width: u32,
        height: u32,
        format: crate::types::TextureFormat,
    ) -> Result<()> {
        let rc = unsafe {
            ffi::nozzle_sender_publish_gl_texture(
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

    pub fn info(&self) -> Result<SenderInfo> {
        let mut info = ffi::NozzleSenderInfo {
            name: ptr::null(),
            application_name: ptr::null(),
            id: ptr::null(),
            backend: 0,
        };
        let rc = unsafe { ffi::nozzle_sender_get_info(self.raw, &mut info) };
        check(rc as _)?;

        Ok(SenderInfo {
            name: crate::cstr_to_string(info.name),
            application_name: crate::cstr_to_string(info.application_name),
            id: crate::cstr_to_string(info.id),
            backend: crate::types::BackendType::from_raw(info.backend as _),
        })
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::nozzle_sender_destroy(self.raw) };
        }
    }
}

unsafe impl Send for Sender {}
unsafe impl Sync for Sender {}
