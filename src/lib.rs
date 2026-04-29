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
    AcquireDesc, BackendType, ConnectedSenderInfo, FrameInfo, FrameStatus,
    ReceiverDesc, SenderDesc, SenderInfo, TextureFormat,
};
