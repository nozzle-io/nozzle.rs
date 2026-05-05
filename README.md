# nozzle.rs

> This codebase is currently in its AI-slob prototyping phase: the code runs on momentum, vibes, and plausible intent.
> Proper debugging will be introduced once demand graduates from hypothetical to measurable.

Rust bindings for [nozzle](https://github.com/nozzle-io/nozzle) — cross-platform GPU texture sharing between local processes.

## Disclaimer / Notice

This library is currently a work in progress and contains many incomplete features and unverified implementations.
Although it may appear usable at first glance, it may not function correctly.

Please use it with the understanding that no guarantees are made regarding its behavior, and perform debugging, validation, and review as needed.
If you encounter problems, please do not become angry; instead, contributions in the form of Issues or Pull Requests would be greatly appreciated.

## Build Requirements

- Rust stable
- C++17 compiler (clang / MSVC)
- CMake 3.20+
- macOS 12+ or Windows 10+

The nozzle C library is built from source via a git submodule. CMake and a C++ compiler are required.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
nozzle = { git = "https://github.com/nozzle-io/nozzle.rs.git" }
```

### Sender

```rust
use nozzle::*;

let mut sender = Sender::create(&SenderDesc {
    name: "my_sender".into(),
    application_name: "MyApp".into(),
    ..Default::default()
})?;

let frame = sender.acquire_writable_frame(1920, 1080, TextureFormat::Rgba8Unorm)?;
// ... write GPU data into frame ...
sender.commit_frame(frame)?;
```

### Receiver

```rust
use nozzle::*;

let mut receiver = Receiver::create(&ReceiverDesc {
    name: "my_sender".into(),
    application_name: "MyViewer".into(),
    ..Default::default()
})?;

let frame = receiver.acquire_frame(&AcquireDesc { timeout_ms: 1000 })?;
let info = frame.info()?;
println!("{}x{} frame #{}", info.width, info.height, info.frame_index);
```

### Discovery

```rust
use nozzle::*;

let senders = enumerate_senders()?;
for s in &senders {
    println!("sender: {} ({} via {:?})", s.name, s.id, s.backend);
}
```

### CPU Pixel Access

```rust
use nozzle::*;

let mut frame = sender.acquire_writable_frame(512, 512, TextureFormat::Rgba8Unorm)?;
{
    let mut pixels = frame.lock_writable_pixels()?;
    for y in 0..pixels.height {
        let row = pixels.row_mut(y).unwrap();
        for b in row.iter_mut() {
            *b = 0xFF;
        }
    }
}
sender.commit_frame(frame)?;
```

## Error Handling

All fallible operations return `nozzle::Result<T>`. Error codes map directly to the C ABI:

```rust
match sender.acquire_writable_frame(0, 0, TextureFormat::Unknown) {
    Err(e) => {
        assert_eq!(e.code, ErrorCode::InvalidArgument);
        eprintln!("{}", e);
    }
    Ok(_) => unreachable!(),
}
```

## Texture Formats

All nozzle formats are exposed as `TextureFormat` variants:

| Format | Bytes/Pixel |
|--------|-------------|
| `R8Unorm` | 1 |
| `RG8Unorm` | 2 |
| `Rgba8Unorm` / `Bgra8Unorm` | 4 |
| `Rgba8Srgb` / `Bgra8Srgb` | 4 |
| `R16Unorm` | 2 |
| `RG16Unorm` | 4 |
| `Rgba16Unorm` | 8 |
| `R16Float` | 2 |
| `RG16Float` | 4 |
| `Rgba16Float` | 8 |
| `R32Float` | 4 |
| `RG32Float` | 8 |
| `Rgba32Float` | 16 |
| `R32Uint` | 4 |
| `Rgba32Uint` | 16 |
| `Depth32Float` | 4 |

## Platform Notes

- **macOS**: Links Metal, IOSurface, Foundation frameworks automatically
- **Windows**: Links d3d11, dxgi, ole32, user32 automatically
- The crate is thread-safe — `Sender` and `Receiver` implement `Send + Sync`

## License

MIT

Third-party dependencies:

- [nozzle](https://github.com/nozzle-io/nozzle) — MIT
