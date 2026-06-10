# nozzle.rs

Experimental pre-1.0 Rust bindings for [nozzle](https://github.com/nozzle-io/nozzle), a cross-platform GPU texture sharing library for local processes.

APIs, packaging, and platform behavior may change before stabilization. Treat this crate as early integration work: validate the target platform/backend before using it in production, and report issues with exact OS, GPU, backend, and nozzle/nozzle.rs versions.

## Build Requirements

- Rust 1.82 or newer
- C++17 compiler (clang, MSVC, or GCC)
- CMake 3.20+
- libclang for bindgen
- One supported native platform:
  - macOS 12+
  - Windows 10+
  - Linux with `libdrm`, `gbm`, `EGL`, and OpenGL development packages

The nozzle C library is built from vendored source in the crate package. When building from a git checkout instead of a published crate, initialize submodules recursively before running Cargo commands.

## Usage

Before the first crates.io publication, use the git dependency:

```toml
[dependencies]
nozzle = { git = "https://github.com/nozzle-io/nozzle.rs.git" }
```

After `nozzle` is published on crates.io, use the registry dependency:

```toml
[dependencies]
nozzle = "0.1"
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

- **macOS**: Links Metal, IOSurface, Foundation, CoreFoundation, Accelerate, OpenGL, Objective-C, and C++ runtime dependencies automatically.
- **Windows**: Links d3d11, dxgi, opengl32, bcrypt, ole32, and user32 automatically.
- **Linux**: Links libdrm, gbm, EGL, OpenGL, and the C++ runtime; install the matching development packages before building.
- The crate is thread-safe — `Sender` and `Receiver` implement `Send + Sync`.

## License

MIT

Third-party dependencies:

- [nozzle](https://github.com/nozzle-io/nozzle) — MIT
