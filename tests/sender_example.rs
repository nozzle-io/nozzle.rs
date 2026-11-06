#[path = "../examples/support/gradient.rs"]
mod gradient;

use nozzle::{ErrorCode, TextureFormat};

#[test]
fn gradient_actual_byte_layout_and_padding() {
    for (format, expected) in [
        (TextureFormat::Rgba8Unorm, [2, 1, 128, 255]),
        (TextureFormat::Bgra8Unorm, [128, 1, 2, 255]),
        (TextureFormat::Rgba8Srgb, [2, 1, 128, 255]),
        (TextureFormat::Bgra8Srgb, [128, 1, 2, 255]),
    ] {
        let mut data = vec![0xcd; 3 * 32];
        gradient::write_gradient(&mut data, 5, 3, 32, format).unwrap();
        assert_eq!(&data[80..84], &expected, "{format}");
        for row in data.chunks_exact(32) {
            assert_eq!(&row[20..], &[0xcd; 12], "padding changed: {format}");
        }
    }
}

#[test]
fn gradient_rejects_unsupported_before_writing() {
    for format in [
        TextureFormat::Unknown,
        TextureFormat::R8Unorm,
        TextureFormat::Rgba16Float,
        TextureFormat::Rgba32Float,
    ] {
        let mut data = vec![0xcd; 128];
        let error = gradient::write_gradient(&mut data, 5, 3, 32, format).unwrap_err();
        assert_eq!(error.code, ErrorCode::UnsupportedFormat);
        assert_eq!(data, vec![0xcd; 128]);
    }
}

#[test]
fn gradient_rejects_invalid_layout_before_writing() {
    for (width, height, stride, len) in [
        (5, 3, 19, 128),
        (5, 3, 32, 83),
        (0, 3, 32, 128),
        (5, 0, 32, 128),
        (u32::MAX, u32::MAX, u32::MAX, 128),
    ] {
        let mut data = vec![0xcd; len];
        let error =
            gradient::write_gradient(&mut data, width, height, stride, TextureFormat::Rgba8Unorm)
                .unwrap_err();
        assert_eq!(error.code, ErrorCode::InvalidArgument);
        assert_eq!(data, vec![0xcd; len]);
    }
}

#[cfg(target_os = "macos")]
mod native {
    use super::gradient;
    use nozzle::{BackendType, Sender, SenderDesc, TextureFormat};
    use std::{ffi::CString, ptr};

    // Independently observe C storage using constants generated from the embedded header.
    #[allow(
        dead_code,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case
    )]
    mod ffi {
        include!(concat!(env!("OUT_DIR"), "/nozzle_raw.rs"));
    }

    struct Observer {
        receiver: *mut ffi::NozzleReceiver,
        frame: *mut ffi::NozzleFrame,
        mapping: *mut ffi::NozzlePixelMapping,
    }

    impl Drop for Observer {
        fn drop(&mut self) {
            unsafe {
                if !self.mapping.is_null() {
                    ffi::nozzle_pixel_mapping_unlock(&mut self.mapping);
                }
                if !self.frame.is_null() {
                    ffi::nozzle_frame_release(self.frame);
                }
                if !self.receiver.is_null() {
                    ffi::nozzle_receiver_destroy(self.receiver);
                }
            }
        }
    }

    #[test]
    fn metal_example_gradient_to_native_observer() {
        for width in [65, 641] {
            let name = format!("rust-gradient-{}-{width}", std::process::id());
            let mut sender = Sender::create(&SenderDesc {
                name: name.clone(),
                ..Default::default()
            })
            .expect("Metal sender is required; no backend skip");
            assert_eq!(sender.info().unwrap().backend, BackendType::Metal);
            let mut frame = sender
                .acquire_writable_frame(width, 3, TextureFormat::Rgba8Unorm)
                .unwrap();
            {
                let mut pixels = frame.lock_writable_pixels().unwrap();
                assert_eq!(pixels.format, TextureFormat::Bgra8Unorm);
                assert_eq!((pixels.width, pixels.height), (width, 3));
                assert!(
                    pixels.row_stride_bytes > width * 4,
                    "exercise actual padded storage"
                );
                let stride = pixels.row_stride_bytes;
                let format = pixels.format;
                gradient::write_gradient(pixels.data_mut(), width, 3, stride, format).unwrap();
                println!("RUST_GRADIENT_NATIVE request=RGBA8 storage=BGRA8 width={width} stride={stride}");
            }
            sender.commit_frame(frame).unwrap();
            let name = CString::new(name).unwrap();
            let mut observer = Observer {
                receiver: ptr::null_mut(),
                frame: ptr::null_mut(),
                mapping: ptr::null_mut(),
            };
            unsafe {
                let desc = ffi::NozzleReceiverDesc {
                    name: name.as_ptr(),
                    application_name: ptr::null(),
                    receive_mode: ffi::NozzleReceiveMode_NOZZLE_RECEIVE_LATEST_ONLY,
                };
                assert_eq!(
                    ffi::nozzle_receiver_create(&desc, &mut observer.receiver),
                    ffi::NozzleErrorCode_NOZZLE_OK
                );
                let acquire = ffi::NozzleAcquireDesc { timeout_ms: 1000 };
                assert_eq!(
                    ffi::nozzle_receiver_acquire_frame(
                        observer.receiver,
                        &acquire,
                        &mut observer.frame
                    ),
                    ffi::NozzleErrorCode_NOZZLE_OK
                );
                let mut pixels: ffi::NozzleMappedPixels = std::mem::zeroed();
                assert_eq!(
                    ffi::nozzle_frame_lock_pixels_mapping_with_origin(
                        observer.frame,
                        ffi::NozzleTextureOrigin_NOZZLE_ORIGIN_TOP_LEFT,
                        &mut observer.mapping,
                        &mut pixels
                    ),
                    ffi::NozzleErrorCode_NOZZLE_OK
                );
                assert_eq!(
                    pixels.format,
                    ffi::NozzleTextureFormat_NOZZLE_FORMAT_BGRA8_UNORM
                );
                assert_eq!(
                    pixels.origin,
                    ffi::NozzleTextureOrigin_NOZZLE_ORIGIN_TOP_LEFT
                );
                assert_eq!((pixels.width, pixels.height), (width, 3));
                assert!(!pixels.data.is_null());
                assert!(pixels.row_stride_bytes >= i64::from(width * 4));
                for y in 0..3usize {
                    let row = std::slice::from_raw_parts(
                        (pixels.data as *const u8).add(y * pixels.row_stride_bytes as usize),
                        width as usize * 4,
                    );
                    for x in 0..width as usize {
                        assert_eq!(
                            &row[x * 4..x * 4 + 4],
                            &[128, (y / 2) as u8, (x / 2) as u8, 255],
                            "native BGRA width={width} x={x} y={y}"
                        );
                    }
                }
                assert_eq!(
                    ffi::nozzle_pixel_mapping_unlock_checked(&mut observer.mapping),
                    ffi::NozzleErrorCode_NOZZLE_OK
                );
            }
        }
    }
}
