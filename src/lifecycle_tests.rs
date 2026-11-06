use crate::frame::{WRAPPERS_ACQUIRED, WRAPPERS_RELEASED};
use crate::{BackendType, ErrorCode, Sender, SenderDesc, TextureFormat};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

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

#[test]
fn mixed_commit_discard_and_drop_release_once() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let acquired = WRAPPERS_ACQUIRED.load(Ordering::Relaxed);
    let released = WRAPPERS_RELEASED.load(Ordering::Relaxed);
    let mut sender = sender("mixed");
    for cycle in 0..12 {
        let mut frame = sender
            .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
            .unwrap();
        {
            let mut pixels = frame.lock_writable_pixels().unwrap();
            pixels.data_mut().fill(0);
        }
        match cycle % 3 {
            0 => sender.commit_frame(frame).unwrap(),
            1 => sender.discard_frame(frame).unwrap(),
            _ => drop(frame),
        }
    }
    assert_eq!(WRAPPERS_ACQUIRED.load(Ordering::Relaxed) - acquired, 12);
    assert_eq!(WRAPPERS_RELEASED.load(Ordering::Relaxed) - released, 12);
    println!("RUST_MIXED commits=4 discards=4 drops=4 released=12");
}

#[test]
fn frames_keep_sender_alive_until_after_wrapper_release() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let released = WRAPPERS_RELEASED.load(Ordering::Relaxed);
    let mut sender = sender("outlive");
    let owner = Arc::downgrade(&sender.owner);
    let first = sender
        .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
        .unwrap();
    let second = sender
        .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
        .unwrap();
    drop(sender);
    assert!(owner.upgrade().is_some());
    assert_eq!(first.info().unwrap().width, 16);
    drop(first);
    assert!(owner.upgrade().is_some());
    assert_eq!(WRAPPERS_RELEASED.load(Ordering::Relaxed) - released, 1);
    drop(second);
    assert!(owner.upgrade().is_none());
    assert_eq!(WRAPPERS_RELEASED.load(Ordering::Relaxed) - released, 2);
    println!("RUST_OWNER public_sender_dropped_first=true released=2 owner_gone=true");
}

#[test]
fn foreign_frames_are_rejected_and_both_senders_remain_usable() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let acquired = WRAPPERS_ACQUIRED.load(Ordering::Relaxed);
    let released = WRAPPERS_RELEASED.load(Ordering::Relaxed);
    let mut original = sender("original");
    let mut target = sender("target");
    for discard in [false, true] {
        let held = target
            .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
            .unwrap();
        let foreign = original
            .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
            .unwrap();
        let result = if discard {
            target.discard_frame(foreign)
        } else {
            target.commit_frame(foreign)
        };
        assert_eq!(result.unwrap_err().code, ErrorCode::InvalidArgument);
        target.commit_frame(held).unwrap();
        for _ in 0..4 {
            for sender in [&mut original, &mut target] {
                let frame = sender
                    .acquire_writable_frame(16, 8, TextureFormat::Rgba8Unorm)
                    .unwrap();
                sender.discard_frame(frame).unwrap();
            }
        }
    }
    assert_eq!(WRAPPERS_ACQUIRED.load(Ordering::Relaxed) - acquired, 20);
    assert_eq!(WRAPPERS_RELEASED.load(Ordering::Relaxed) - released, 20);
    println!("RUST_FOREIGN commit_and_discard_rejected=true both_senders_usable=true released=20");
}
