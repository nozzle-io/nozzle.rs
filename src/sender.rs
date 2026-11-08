use crate::error::{check, Error, ErrorCode, Result};
use crate::ffi;
use crate::frame::WritableFrame;
use crate::types::{SenderDesc, SenderInfo};

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::Arc;

pub struct Sender {
    pub(crate) owner: Arc<SenderOwner>,
}

pub(crate) struct SenderOwner {
    pub(crate) raw: *mut ffi::NozzleSender,
}

impl Drop for SenderOwner {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::nozzle_sender_destroy(self.raw) };
        }
    }
}

// Preserve Sender's existing cross-thread contract while sharing its lifetime.
unsafe impl Send for SenderOwner {}
unsafe impl Sync for SenderOwner {}

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

        Ok(Sender {
            owner: Arc::new(SenderOwner { raw: sender_ptr }),
        })
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
                self.owner.raw,
                width,
                height,
                format as _,
                &mut frame_ptr,
            )
        };
        check(rc as _)?;
        Ok(WritableFrame::from_raw(frame_ptr, Arc::clone(&self.owner)))
    }

    /// Consumes the frame even on error. A failed commit is discarded once,
    /// then the original commit error is returned and the C wrapper is released.
    pub fn commit_frame(&mut self, mut frame: WritableFrame) -> Result<()> {
        if !Arc::ptr_eq(&self.owner, &frame.owner) {
            return Err(Error::with_message(
                ErrorCode::InvalidArgument,
                "writable frame belongs to another sender",
            ));
        }
        let rc = unsafe { ffi::nozzle_sender_commit_frame(self.owner.raw, frame.as_raw()) };
        if rc != ffi::NozzleErrorCode_NOZZLE_OK {
            // Some commit failures already consumed the payload. Discard may
            // then return InvalidArgument; preserve the primary commit error.
            unsafe { ffi::nozzle_sender_discard_frame(self.owner.raw, frame.as_raw()) };
        }
        frame.needs_discard = false;
        check(rc as _)
    }

    /// Consumes an unpublished frame and reports the native discard result.
    /// A foreign frame is rejected and cleaned up through its original sender.
    pub fn discard_frame(&mut self, mut frame: WritableFrame) -> Result<()> {
        if !Arc::ptr_eq(&self.owner, &frame.owner) {
            return Err(Error::with_message(
                ErrorCode::InvalidArgument,
                "writable frame belongs to another sender",
            ));
        }
        let rc = unsafe { ffi::nozzle_sender_discard_frame(self.owner.raw, frame.as_raw()) };
        frame.needs_discard = false;
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
                self.owner.raw,
                gl_texture_name,
                gl_target,
                width,
                height,
                format as _,
            )
        };
        check(rc as _)
    }

    /// Returns owned metadata that remains valid after later calls or sender destruction.
    pub fn info(&self) -> Result<SenderInfo> {
        let mut info: ffi::NozzleSenderInfoSnapshot = unsafe { mem::zeroed() };
        let rc = unsafe { ffi::nozzle_sender_get_info_snapshot(self.owner.raw, &mut info) };
        check(rc as _)?;

        Ok(SenderInfo {
            name: crate::fixed_cstr_to_string(&info.name),
            application_name: crate::fixed_cstr_to_string(&info.application_name),
            id: crate::fixed_cstr_to_string(&info.id),
            backend: crate::types::BackendType::from_raw(info.backend as _),
        })
    }
}
