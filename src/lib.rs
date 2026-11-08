mod discovery;
mod error;
mod frame;
pub mod pixel;
mod receiver;
mod sender;
mod types;

#[cfg(all(test, target_os = "macos"))]
mod lifecycle_tests;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/nozzle_raw.rs"));
}

pub use discovery::enumerate_senders;
pub use error::{Error, ErrorCode, Result};
pub use frame::{Frame, MappedPixels, WritableFrame};
pub use receiver::Receiver;
pub use sender::Sender;
pub use types::{
    AcquireDesc, BackendType, ConnectedSenderInfo, ConnectionSnapshot, FormatSource, FrameInfo,
    FrameStatus, NativeFormatKind, ReceiveMode, ReceiverDesc, ResolvedTextureFormat, SenderDesc,
    SenderInfo, SyncMode, TextureFormat, TextureOrigin, TextureWrapDesc, TransferMode,
};

pub(crate) fn cstr_to_string(ptr: *const std::os::raw::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

pub(crate) fn fixed_cstr_to_string<const N: usize>(chars: &[std::os::raw::c_char; N]) -> String {
    let bytes = chars.map(|ch| ch as u8);
    let end = bytes.iter().position(|&byte| byte == 0).unwrap_or(N);
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

#[cfg(test)]
mod info_string_tests {
    use super::fixed_cstr_to_string;
    use std::os::raw::c_char;

    #[test]
    fn empty_array_and_empty_string() {
        assert_eq!(fixed_cstr_to_string(&[]), "");
        assert_eq!(fixed_cstr_to_string(&[0, b'x' as c_char]), "");
    }

    #[test]
    fn stops_at_first_nul() {
        let chars = b"name\0ignored".map(|byte| byte as c_char);
        assert_eq!(fixed_cstr_to_string(&chars), "name");
    }

    #[test]
    fn full_array_without_nul_is_bounded() {
        let chars = b"full".map(|byte| byte as c_char);
        assert_eq!(fixed_cstr_to_string(&chars), "full");
    }

    #[test]
    fn invalid_utf8_remains_lossy() {
        let chars = b"a\xffb".map(|byte| byte as c_char);
        assert_eq!(fixed_cstr_to_string(&chars), "a\u{fffd}b");
    }
}
