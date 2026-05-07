use crate::error::{check, Result};
use crate::ffi;
use crate::frame::Frame;
use crate::types::{AcquireDesc, ConnectedSenderInfo, ReceiverDesc};

use std::ffi::CString;
use std::ptr;

pub struct Receiver {
    raw: *mut ffi::NozzleReceiver,
}

impl Receiver {
    pub fn create(desc: &ReceiverDesc) -> Result<Self> {
        let name = CString::new(&desc.name[..]).map_err(|_| {
            crate::error::Error::with_message(
                crate::ErrorCode::InvalidArgument,
                "receiver name contains nul byte",
            )
        })?;
        let app_name = CString::new(&desc.application_name[..]).map_err(|_| {
            crate::error::Error::with_message(
                crate::ErrorCode::InvalidArgument,
                "application name contains nul byte",
            )
        })?;

        let c_desc = ffi::NozzleReceiverDesc {
            name: name.as_ptr(),
            application_name: app_name.as_ptr(),
            receive_mode: desc.receive_mode as _,
        };

        let mut receiver_ptr: *mut ffi::NozzleReceiver = ptr::null_mut();
        let rc = unsafe { ffi::nozzle_receiver_create(&c_desc, &mut receiver_ptr) };
        check(rc as _)?;

        Ok(Receiver { raw: receiver_ptr })
    }

    pub fn acquire_frame(&mut self, desc: &AcquireDesc) -> Result<Frame> {
        let c_desc = ffi::NozzleAcquireDesc {
            timeout_ms: desc.timeout_ms,
        };

        let mut frame_ptr: *mut ffi::NozzleFrame = ptr::null_mut();
        let rc = unsafe { ffi::nozzle_receiver_acquire_frame(self.raw, &c_desc, &mut frame_ptr) };
        check(rc as _)?;
        Ok(Frame::from_raw(frame_ptr))
    }

    pub fn connected_info(&self) -> Result<ConnectedSenderInfo> {
        let mut info = ffi::NozzleConnectedSenderInfo {
            name: ptr::null(),
            application_name: ptr::null(),
            id: ptr::null(),
            backend: 0,
            width: 0,
            height: 0,
            format: 0,
            semantic_format: 0,
            estimated_fps: 0.0,
            frame_counter: 0,
            last_update_time_ns: 0,
            native_format_modifier: 0,
        };
        let rc = unsafe { ffi::nozzle_receiver_get_connected_info(self.raw, &mut info) };
        check(rc as _)?;

        Ok(ConnectedSenderInfo {
            name: crate::cstr_to_string(info.name),
            application_name: crate::cstr_to_string(info.application_name),
            id: crate::cstr_to_string(info.id),
            backend: crate::types::BackendType::from_raw(info.backend as _),
            width: info.width,
            height: info.height,
            format: crate::types::TextureFormat::from_raw(info.format as _),
            semantic_format: crate::types::TextureFormat::from_raw(info.semantic_format as _),
            estimated_fps: info.estimated_fps,
            frame_counter: info.frame_counter,
            last_update_time_ns: info.last_update_time_ns,
            native_format_modifier: info.native_format_modifier,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.connected_info().is_ok()
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::nozzle_receiver_destroy(self.raw) };
        }
    }
}

unsafe impl Send for Receiver {}
unsafe impl Sync for Receiver {}
