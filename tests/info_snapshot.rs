//! Real same-process Metal metadata checks, not pixel or cross-process evidence.
//! These use the public API; no synthetic native handles/counters.
#![cfg(target_os = "macos")]

use nozzle::{AcquireDesc, BackendType, ErrorCode, Receiver, ReceiverDesc, Sender, SenderDesc};

fn create_pair(case: &str) -> (Sender, Receiver, String, String) {
    let name = format!("rust-info-{}-{case}-\u{1f680}", std::process::id());
    let application = "snapshot-\u{65e5}\u{672c}-\u{1f3a5}".to_owned();
    let sender = Sender::create(&SenderDesc {
        name: name.clone(),
        application_name: application.clone(),
        ..Default::default()
    })
    .expect("real Metal sender required; no backend skip");
    assert_eq!(
        sender.info().expect("sender metadata").backend,
        BackendType::Metal
    );
    let receiver = Receiver::create(&ReceiverDesc {
        name: name.clone(),
        ..Default::default()
    })
    .expect("connect to real sender");
    (sender, receiver, name, application)
}

#[test]
fn observed_sender_closure_is_disconnected_with_last_known_identity() {
    let (sender, mut receiver, name, application) = create_pair("closed");
    let retained = receiver
        .connected_info()
        .expect("initial connected metadata");
    let sender_info = sender.info().expect("initial sender metadata");
    assert!(receiver.is_connected());
    let initial = receiver.connection_snapshot().expect("initial snapshot");
    assert!(initial.connected);
    assert_eq!(initial.info.name, name);
    assert_eq!(initial.info.application_name, application);
    assert_eq!(initial.info.id, sender_info.id);
    assert_eq!(retained.id, sender_info.id);
    assert!(!retained.id.is_empty());

    // No writable frames retain SenderOwner. Core observes the closed marker
    // before its frame wait, even for timeout_ms=0, without process death/sleep.
    drop(sender);
    match receiver.acquire_frame(&AcquireDesc { timeout_ms: 0 }) {
        Err(error) => {
            println!("RUST_INFO_CLOSURE acquire_error={:?}", error.code);
            assert_eq!(error.code, ErrorCode::SenderClosed);
        }
        Ok(_) => panic!("destroyed sender unexpectedly supplied a frame"),
    }
    let last_known = receiver.connected_info().expect("last-known metadata");
    assert_eq!(last_known.name, name);
    assert_eq!(last_known.application_name, application);
    assert_eq!(last_known.id, retained.id);
    assert_eq!(last_known.backend, BackendType::Metal);
    assert_eq!(retained.name, name);
    assert_eq!(retained.application_name, application);

    let connected = receiver.is_connected();
    println!(
        "RUST_INFO_CLOSURE last_known_id={} connected={connected}",
        last_known.id
    );
    assert!(!connected, "observed SenderClosed must report disconnected");
    let closed = receiver.connection_snapshot().expect("closed snapshot");
    assert!(!closed.connected);
    assert_eq!(closed.info.name, name);
    assert_eq!(closed.info.application_name, application);
    assert_eq!(closed.info.id, initial.info.id);
    assert_eq!(closed.info.backend, BackendType::Metal);
    assert!(initial.connected, "retained snapshot must not change");
}

#[test]
fn metadata_strings_remain_owned_after_later_calls_and_handle_destruction() {
    let (sender, receiver, name, application) = create_pair("owned");
    let sender_info = sender.info().expect("owned sender metadata");
    let receiver_info = receiver.connected_info().expect("owned receiver metadata");
    let snapshot = receiver.connection_snapshot().expect("owned snapshot");
    assert_eq!(sender.info().expect("later sender call").id, sender_info.id);
    assert_eq!(
        receiver.connected_info().expect("later receiver call").id,
        receiver_info.id
    );
    drop(receiver);
    drop(sender);

    assert_eq!(sender_info.name, name);
    assert_eq!(sender_info.application_name, application);
    assert_eq!(receiver_info.name, name);
    assert_eq!(receiver_info.application_name, application);
    assert_eq!(receiver_info.id, sender_info.id);
    assert!(!sender_info.id.is_empty());
    assert_eq!(sender_info.backend, BackendType::Metal);
    assert_eq!(receiver_info.backend, BackendType::Metal);
    assert!(snapshot.connected);
    assert_eq!(snapshot.info.name, name);
    assert_eq!(snapshot.info.application_name, application);
    assert_eq!(snapshot.info.id, sender_info.id);
    println!("RUST_INFO_OWNED PASS retained UTF-8 identity after both handles dropped");
}
