use nozzle::*;
use std::thread;
use std::time::{Duration, Instant};

#[path = "support/gradient.rs"]
mod gradient;

fn main() -> nozzle::Result<()> {
    let mut sender = Sender::create(&SenderDesc {
        name: "rust_sender".into(),
        application_name: "RustExample".into(),
        ..Default::default()
    })?;

    println!("sender created, streaming 512x512 gradient for 30 seconds...");

    let total_frames = 300u64;
    let start = Instant::now();
    let mut published = 0u64;

    for i in 0..total_frames {
        let result = (|| -> nozzle::Result<()> {
            let mut frame = sender.acquire_writable_frame(512, 512, TextureFormat::Rgba8Unorm)?;
            {
                let mut pixels = frame.lock_writable_pixels()?;
                let (width, height, stride, format) = (
                    pixels.width,
                    pixels.height,
                    pixels.row_stride_bytes,
                    pixels.format,
                );
                gradient::write_gradient(pixels.data_mut(), width, height, stride, format)?;
            }
            // MappedPixels auto-unlocks on drop
            sender.commit_frame(frame)
        })();
        match result {
            Ok(()) => published += 1,
            Err(error) if error.code == ErrorCode::Timeout => {}
            Err(error) => return Err(error),
        }

        if i % 10 == 0 {
            let elapsed = start.elapsed().as_secs();
            println!("published {} frames ({}s)", published, elapsed);
        }

        // Busy-ring drops use the same pacing as successful publications.
        thread::sleep(Duration::from_millis(100));
    }

    println!(
        "done. {} frames sent in {} attempts.",
        published, total_frames
    );
    Ok(())
}
