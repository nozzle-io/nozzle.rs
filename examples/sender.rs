use nozzle::*;
use std::thread;
use std::time::{Duration, Instant};

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
            let row_bytes = pixels.row_stride_bytes as usize;
            let w = pixels.width as usize;
            let h = pixels.height as usize;
            let data = pixels.data_mut();

            for y in 0..h {
                let row_start = y * row_bytes;
                for x in 0..w {
                    let offset = row_start + x * 4;
                    data[offset] = (x / 2) as u8;
                    data[offset + 1] = (y / 2) as u8;
                    data[offset + 2] = 128;
                    data[offset + 3] = 255;
                }
            }
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
