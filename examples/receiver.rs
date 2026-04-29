use nozzle::*;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> nozzle::Result<()> {
    let mut receiver = Receiver::create(&ReceiverDesc {
        name: "rust_sender".into(),
        application_name: "RustViewer".into(),
        ..Default::default()
    })?;

    println!("receiver listening for 'rust_sender' for 30 seconds...");

    let start = Instant::now();
    let mut frame_count = 0u64;

    while start.elapsed() < Duration::from_secs(30) {
        let frame = match receiver.acquire_frame(&AcquireDesc { timeout_ms: 1000 }) {
            Ok(f) => f,
            Err(_) => {
                println!("no sender found yet, waiting...");
                continue;
            }
        };

        let info = frame.info()?;
        frame_count += 1;

        print!(
            "frame #{}: {}x{} format={}",
            info.frame_index, info.width, info.height, info.format
        );

        if let Ok(mut pixels) = frame.lock_pixels() {
            let data = pixels.data();
            if data.len() >= 4 {
                print!(
                    " first_4_bytes=[{}, {}, {}, {}]",
                    data[0], data[1], data[2], data[3]
                );
            }
        }

        println!();

        thread::sleep(Duration::from_millis(33));
    }

    println!("done. {} frames received.", frame_count);
    Ok(())
}
