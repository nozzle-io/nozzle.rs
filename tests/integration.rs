use nozzle::*;

#[test]
fn test_sender_create_destroy() {
    let desc = SenderDesc {
        name: "test_sender".to_owned(),
        application_name: "test_app".to_owned(),
        ..Default::default()
    };

    let sender = Sender::create(&desc);
    assert!(sender.is_ok(), "sender create failed: {:?}", sender.err());
    let sender = sender.unwrap();

    let info = sender.info().expect("sender info");
    assert_eq!(info.name, "test_sender");
    assert_eq!(info.application_name, "test_app");
    assert!(!info.id.is_empty());
}

#[test]
fn test_receiver_create_destroy() {
    let desc = ReceiverDesc {
        name: "nonexistent_sender".to_owned(),
        application_name: "test_viewer".to_owned(),
        ..Default::default()
    };

    let receiver = Receiver::create(&desc);
    assert!(receiver.is_ok(), "receiver create failed: {:?}", receiver.err());
}

#[test]
fn test_sender_acquire_writable_frame() {
    let desc = SenderDesc {
        name: "frame_test".to_owned(),
        application_name: "frame_app".to_owned(),
        ..Default::default()
    };

    let mut sender = Sender::create(&desc).expect("sender create");

    let frame = sender.acquire_writable_frame(64, 64, TextureFormat::Rgba8Unorm);
    assert!(frame.is_ok(), "acquire writable frame failed: {:?}", frame.err());

    let frame = frame.unwrap();
    let info = frame.info().expect("frame info");
    assert_eq!(info.width, 64);
    assert_eq!(info.height, 64);
    assert_eq!(info.format, TextureFormat::Rgba8Unorm);
}

#[test]
fn test_sender_commit_frame() {
    let desc = SenderDesc {
        name: "commit_test".to_owned(),
        application_name: "commit_app".to_owned(),
        ..Default::default()
    };

    let mut sender = Sender::create(&desc).expect("sender create");

    let frame = sender
        .acquire_writable_frame(32, 32, TextureFormat::R8Unorm)
        .expect("acquire writable frame");

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
    assert!(display.contains("timeout"), "error display: {}", display);
}

#[test]
fn test_enumerate_senders() {
    let senders = enumerate_senders().expect("enumerate senders");
    // no guarantee of senders existing, but the call itself should not fail
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

    let sender = Sender::create(&desc).expect("sender create");
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
    assert!(sender_a.is_ok());
}
