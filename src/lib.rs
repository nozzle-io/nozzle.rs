#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;
mod discovery;
mod frame;
mod pixel;
mod receiver;
mod sender;
mod types;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/nozzle_raw.rs"));
}

pub use error::{Error, ErrorCode, Result};
pub use discovery::enumerate_senders;
pub use frame::{Frame, MappedPixels, WritableFrame};
pub use receiver::Receiver;
pub use sender::Sender;
pub use types::{
    AcquireDesc, BackendType, ConnectedSenderInfo, FormatSource, FrameInfo, FrameStatus,
    NativeFormatKind, ReceiveMode, ReceiverDesc, ResolvedTextureFormat, SenderDesc,
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
