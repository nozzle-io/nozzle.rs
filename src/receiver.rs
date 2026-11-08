use crate::error::{check, Result};
use crate::ffi;
use crate::frame::Frame;
use crate::types::{AcquireDesc, ConnectedSenderInfo, ConnectionSnapshot, ReceiverDesc};

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

    /// Returns owned last-known metadata, including after an observed disconnect.
    pub fn connected_info(&self) -> Result<ConnectedSenderInfo> {
        self.connection_snapshot().map(|snapshot| snapshot.info)
    }

    /// Copies connection state and metadata together under the native receiver lock.
    /// The result owns its strings and survives later calls and receiver destruction.
    /// This reports last-observed state, not fresh liveness or a reconnect attempt.
    pub fn connection_snapshot(&self) -> Result<ConnectionSnapshot> {
        let mut info: ffi::NozzleConnectedSenderSnapshot = unsafe { std::mem::zeroed() };
        let rc = unsafe { ffi::nozzle_receiver_get_connected_info_snapshot(self.raw, &mut info) };
        check(rc as _)?;

        Ok(snapshot_from_native(&info))
    }

    /// Returns last-observed connection state, or false if snapshot retrieval fails.
    /// The sender may have exited since the receiver last observed it.
    pub fn is_connected(&self) -> bool {
        self.connection_snapshot()
            .map(|snapshot| snapshot.connected)
            .unwrap_or(false)
    }
}

fn snapshot_from_native(info: &ffi::NozzleConnectedSenderSnapshot) -> ConnectionSnapshot {
    ConnectionSnapshot {
        connected: info.connected != 0,
        info: ConnectedSenderInfo {
            name: crate::fixed_cstr_to_string(&info.name),
            application_name: crate::fixed_cstr_to_string(&info.application_name),
            id: crate::fixed_cstr_to_string(&info.id),
            backend: crate::types::BackendType::from_raw(info.backend as _),
            width: info.width,
            height: info.height,
            format: crate::types::TextureFormat::from_raw(info.format as _),
            semantic_format: crate::types::TextureFormat::from_raw(info.semantic_format as _),
            estimated_fps: info.estimated_fps,
            frame_counter: info.frame_counter,
            last_update_time_ns: info.last_update_time_ns,
            native_format_kind: crate::types::NativeFormatKind::from_raw(
                info.native_format_kind as _,
            ),
            native_format_value: info.native_format_value,
            native_format_modifier: info.native_format_modifier,
        },
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

#[cfg(test)]
mod snapshot_tests {
    use super::{ffi, snapshot_from_native};
    use crate::{BackendType, NativeFormatKind, TextureFormat};
    use std::os::raw::c_char;

    fn c_array<const N: usize>(bytes: &[u8]) -> [c_char; N] {
        assert!(bytes.len() < N);
        let mut chars = [0; N];
        for (dst, &byte) in chars.iter_mut().zip(bytes) {
            *dst = byte as c_char;
        }
        chars
    }

    #[test]
    fn conversion_preserves_state_identity_and_all_metadata() {
        // Synthetic conversion only: no native call or shared lifecycle counter.
        let mut raw = ffi::NozzleConnectedSenderSnapshot {
            name: c_array(b"last-name"),
            application_name: c_array(b"last-application"),
            id: c_array(b"last-id"),
            connected: 1,
            backend: BackendType::Metal as _,
            width: 65,
            height: 3,
            format: TextureFormat::Rgba16Float as _,
            semantic_format: TextureFormat::Rgb16Float as _,
            estimated_fps: 59.25,
            frame_counter: (1_u64 << 40) + 7,
            last_update_time_ns: (1_u64 << 60) + 9,
            native_format_kind: NativeFormatKind::MtlPixelFormat as _,
            native_format_value: 115,
            native_format_modifier: u64::MAX - 3,
        };
        let connected = snapshot_from_native(&raw);
        assert!(connected.connected);
        raw.connected = 0;
        let disconnected = snapshot_from_native(&raw);
        assert!(!disconnected.connected);
        raw.name.fill(0);
        raw.application_name.fill(0);
        raw.id.fill(0);

        for snapshot in [connected, disconnected] {
            let info = snapshot.info;
            assert_eq!(info.name, "last-name");
            assert_eq!(info.application_name, "last-application");
            assert_eq!(info.id, "last-id");
            assert_eq!(info.backend, BackendType::Metal);
            assert_eq!(info.width, 65);
            assert_eq!(info.height, 3);
            assert_eq!(info.format, TextureFormat::Rgba16Float);
            assert_eq!(info.semantic_format, TextureFormat::Rgb16Float);
            assert_eq!(info.estimated_fps, 59.25);
            assert_eq!(info.frame_counter, (1_u64 << 40) + 7);
            assert_eq!(info.last_update_time_ns, (1_u64 << 60) + 9);
            assert_eq!(info.native_format_kind, NativeFormatKind::MtlPixelFormat);
            assert_eq!(info.native_format_value, 115);
            assert_eq!(info.native_format_modifier, u64::MAX - 3);
        }
    }
}
