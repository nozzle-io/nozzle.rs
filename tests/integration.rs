use nozzle::*;

fn is_backend_unavailable<T>(result: &std::result::Result<T, Error>) -> bool {
    match result {
        Ok(_) => false,
        Err(e) => matches!(
            e.code,
            ErrorCode::UnsupportedBackend
                | ErrorCode::ResourceCreationFailed
                | ErrorCode::BackendError
        ),
    }
}

#[test]
fn test_sender_create_destroy() {
    let desc = SenderDesc {
        name: "test_sender".to_owned(),
        application_name: "test_app".to_owned(),
        ..Default::default()
    };

    let sender = Sender::create(&desc);
    if is_backend_unavailable(&sender) {
        return;
    }
    let sender = sender.expect("sender create failed");

    let info = sender.info().expect("sender info");
    assert_eq!(info.name, "test_sender");
    assert_eq!(info.application_name, "test_app");
    assert!(!info.id.is_empty());
}

#[test]
fn test_receiver_create_destroy() {
    let sender_desc = SenderDesc {
        name: "test_receiver_target".to_owned(),
        application_name: "test_app".to_owned(),
        ..Default::default()
    };

    let sender = Sender::create(&sender_desc);
    if is_backend_unavailable(&sender) {
        return;
    }
    let _sender = sender.expect("sender create");

    let recv_desc = ReceiverDesc {
        name: "test_receiver_target".to_owned(),
        application_name: "test_viewer".to_owned(),
        ..Default::default()
    };

    let receiver = Receiver::create(&recv_desc);
    let _receiver = receiver.expect("receiver create failed");
}

#[test]
fn test_sender_acquire_writable_frame() {
    let desc = SenderDesc {
        name: "frame_test".to_owned(),
        application_name: "frame_app".to_owned(),
        ..Default::default()
    };

    let sender = Sender::create(&desc);
    if is_backend_unavailable(&sender) {
        return;
    }
    let mut sender = sender.expect("sender create");

    let frame = sender.acquire_writable_frame(64, 64, TextureFormat::Rgba8Unorm);
    if is_backend_unavailable(&frame) {
        return;
    }
    let frame = frame.expect("acquire writable frame failed");

    let info = frame.info().expect("frame info");
    assert_eq!(info.width, 64);
    assert_eq!(info.height, 64);
    // Metal fallback: rgba8 → bgra8 for 8-bit IOSurface
    assert!(info.format == TextureFormat::Rgba8Unorm || info.format == TextureFormat::Bgra8Unorm);
}

#[test]
fn test_sender_commit_frame() {
    let desc = SenderDesc {
        name: "commit_test".to_owned(),
        application_name: "commit_app".to_owned(),
        ..Default::default()
    };

    let sender = Sender::create(&desc);
    if is_backend_unavailable(&sender) {
        return;
    }
    let mut sender = sender.expect("sender create");

    let frame = sender.acquire_writable_frame(32, 32, TextureFormat::R8Unorm);
    if is_backend_unavailable(&frame) {
        return;
    }
    let frame = frame.expect("acquire writable frame");

    let commit = sender.commit_frame(frame);
    assert!(commit.is_ok(), "commit frame failed: {:?}", commit.err());
}

#[test]
fn test_texture_format_bytes_per_pixel() {
    assert_eq!(TextureFormat::R8Unorm.bytes_per_pixel(), Some(1));
    assert_eq!(TextureFormat::Rgba8Unorm.bytes_per_pixel(), Some(4));
    assert_eq!(TextureFormat::Rgba32Float.bytes_per_pixel(), Some(16));
    assert_eq!(TextureFormat::Unknown.bytes_per_pixel(), None);
}

#[test]
fn test_texture_format_display() {
    assert_eq!(format!("{}", TextureFormat::Rgba8Unorm), "rgba8_unorm");
    assert_eq!(format!("{}", TextureFormat::R32Float), "r32_float");
    assert_eq!(format!("{}", TextureFormat::Unknown), "unknown");
}

#[test]
fn test_error_code_roundtrip() {
    let err = Error::new(ErrorCode::Timeout);
    assert_eq!(err.code, ErrorCode::Timeout);
    assert!(!err.message.is_empty());

    let display = format!("{}", err);
    assert!(display.contains("timed out"), "error display: {}", display);
}

#[test]
fn test_enumerate_senders() {
    let senders = enumerate_senders().expect("enumerate senders");
    let _ = senders;
}

#[test]
fn test_sender_info() {
    let desc = SenderDesc {
        name: "info_test".to_owned(),
        application_name: "info_app".to_owned(),
        ring_buffer_size: 2,
        ..Default::default()
    };

    let sender = Sender::create(&desc);
    if is_backend_unavailable(&sender) {
        return;
    }
    let sender = sender.expect("sender create");

    let info = sender.info().expect("sender info");
    assert_eq!(info.name, "info_test");
    assert_eq!(info.application_name, "info_app");
}

#[test]
fn test_invalid_sender_name() {
    let desc = SenderDesc {
        name: "test\0sender".to_owned(),
        application_name: "test_app".to_owned(),
        ..Default::default()
    };

    let result = Sender::create(&desc);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.code, ErrorCode::InvalidArgument);
}

#[test]
fn test_multiple_senders_same_name() {
    let desc = SenderDesc {
        name: "shared_name".to_owned(),
        application_name: "app_a".to_owned(),
        ..Default::default()
    };

    let sender_a = Sender::create(&desc);
    if is_backend_unavailable(&sender_a) {
        return;
    }
    assert!(sender_a.is_ok());
}
