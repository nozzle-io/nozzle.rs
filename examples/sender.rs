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

    for i in 0..total_frames {
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

        sender.commit_frame(frame)?;

        if i % 10 == 0 {
            let elapsed = start.elapsed().as_secs();
            println!("frame {} ({}s)", i, elapsed);
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("done. {} frames sent.", total_frames);
    Ok(())
}
