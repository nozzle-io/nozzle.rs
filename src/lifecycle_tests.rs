use crate::frame::{WRAPPERS_ACQUIRED, WRAPPERS_RELEASED};
use crate::{BackendType, Sender, SenderDesc, TextureFormat};
use std::sync::atomic::Ordering;
use std::sync::Mutex;

// These native tests share allocation counters, not production synchronization.
static SERIAL: Mutex<()> = Mutex::new(());

fn sender(suffix: &str) -> Sender {
    let sender = Sender::create(&SenderDesc {
        name: format!("rust-lifecycle-{}-{suffix}", std::process::id()),
        ring_buffer_size: 2,
        ..Default::default()
    })
    .expect("native Metal lifecycle evidence requires a sender; no backend skip");
    assert_eq!(sender.info().unwrap().backend, BackendType::Metal);
    sender
}

#[test]
fn committed_wrappers_are_released() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let acquired = WRAPPERS_ACQUIRED.load(Ordering::Relaxed);
    let released = WRAPPERS_RELEASED.load(Ordering::Relaxed);
    let mut sender = sender("commit");
    for _ in 0..8 {
        let frame = sender
            .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
            .expect("Metal writable acquisition is required");
        sender.commit_frame(frame).unwrap();
    }
    let acquired = WRAPPERS_ACQUIRED.load(Ordering::Relaxed) - acquired;
    let released = WRAPPERS_RELEASED.load(Ordering::Relaxed) - released;
    println!("RUST_WRAPPER_COUNTS acquired={acquired} released={released}");
    assert_eq!(acquired, 8);
    assert_eq!(released, acquired, "commit must release every C wrapper");
}

#[test]
fn abandoned_frames_return_ring_slots() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let mut sender = sender("abandon");
    for cycle in 0..8 {
        let frame = sender
            .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
            .unwrap_or_else(|error| panic!("two-slot ring exhausted at cycle {cycle}: {error}"));
        drop(frame);
    }
    println!("RUST_ABANDON ring=2 cycles=8");
}
