#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BackendType {
    Unknown = 0,
    D3D11 = 1,
    Metal = 2,
    OpenGL = 3,
}

impl BackendType {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => BackendType::D3D11,
            2 => BackendType::Metal,
            3 => BackendType::OpenGL,
            _ => BackendType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextureFormat {
    Unknown = 0,
    R8Unorm = 1,
    RG8Unorm = 2,
    Rgba8Unorm = 3,
    Bgra8Unorm = 4,
    Rgba8Srgb = 5,
    Bgra8Srgb = 6,
    R16Unorm = 7,
    RG16Unorm = 8,
    Rgba16Unorm = 9,
    R16Float = 10,
    RG16Float = 11,
    Rgba16Float = 12,
    R32Float = 13,
    RG32Float = 14,
    Rgba32Float = 15,
    R32Uint = 16,
    Rgba32Uint = 17,
    Depth32Float = 18,
}

impl TextureFormat {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => TextureFormat::R8Unorm,
            2 => TextureFormat::RG8Unorm,
            3 => TextureFormat::Rgba8Unorm,
            4 => TextureFormat::Bgra8Unorm,
            5 => TextureFormat::Rgba8Srgb,
            6 => TextureFormat::Bgra8Srgb,
            7 => TextureFormat::R16Unorm,
            8 => TextureFormat::RG16Unorm,
            9 => TextureFormat::Rgba16Unorm,
            10 => TextureFormat::R16Float,
            11 => TextureFormat::RG16Float,
            12 => TextureFormat::Rgba16Float,
            13 => TextureFormat::R32Float,
            14 => TextureFormat::RG32Float,
            15 => TextureFormat::Rgba32Float,
            16 => TextureFormat::R32Uint,
            17 => TextureFormat::Rgba32Uint,
            18 => TextureFormat::Depth32Float,
            _ => TextureFormat::Unknown,
        }
    }

    pub fn bytes_per_pixel(&self) -> Option<u32> {
        match self {
            TextureFormat::R8Unorm => Some(1),
            TextureFormat::RG8Unorm => Some(2),
            TextureFormat::Rgba8Unorm => Some(4),
            TextureFormat::Bgra8Unorm => Some(4),
            TextureFormat::Rgba8Srgb => Some(4),
            TextureFormat::Bgra8Srgb => Some(4),
            TextureFormat::R16Unorm => Some(2),
            TextureFormat::RG16Unorm => Some(4),
            TextureFormat::Rgba16Unorm => Some(8),
            TextureFormat::R16Float => Some(2),
            TextureFormat::RG16Float => Some(4),
            TextureFormat::Rgba16Float => Some(8),
            TextureFormat::R32Float => Some(4),
            TextureFormat::RG32Float => Some(8),
            TextureFormat::Rgba32Float => Some(16),
            TextureFormat::R32Uint => Some(4),
            TextureFormat::Rgba32Uint => Some(16),
            TextureFormat::Depth32Float => Some(4),
            TextureFormat::Unknown => None,
        }
    }
}

impl std::fmt::Display for TextureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TextureFormat::Unknown => "unknown",
            TextureFormat::R8Unorm => "r8_unorm",
            TextureFormat::RG8Unorm => "rg8_unorm",
            TextureFormat::Rgba8Unorm => "rgba8_unorm",
            TextureFormat::Bgra8Unorm => "bgra8_unorm",
            TextureFormat::Rgba8Srgb => "rgba8_srgb",
            TextureFormat::Bgra8Srgb => "bgra8_srgb",
            TextureFormat::R16Unorm => "r16_unorm",
            TextureFormat::RG16Unorm => "rg16_unorm",
            TextureFormat::Rgba16Unorm => "rgba16_unorm",
            TextureFormat::R16Float => "r16_float",
            TextureFormat::RG16Float => "rg16_float",
            TextureFormat::Rgba16Float => "rgba16_float",
            TextureFormat::R32Float => "r32_float",
            TextureFormat::RG32Float => "rg32_float",
            TextureFormat::Rgba32Float => "rgba32_float",
            TextureFormat::R32Uint => "r32_uint",
            TextureFormat::Rgba32Uint => "rgba32_uint",
            TextureFormat::Depth32Float => "depth32_float",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReceiveMode {
    LatestOnly = 0,
    SequentialBestEffort = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FrameStatus {
    New = 0,
    NoNew = 1,
    Dropped = 2,
    SenderClosed = 3,
    Error = 4,
}

#[derive(Debug, Clone)]
pub struct SenderInfo {
    pub name: String,
    pub application_name: String,
    pub id: String,
    pub backend: BackendType,
}

#[derive(Debug, Clone)]
pub struct ConnectedSenderInfo {
    pub name: String,
    pub application_name: String,
    pub id: String,
    pub backend: BackendType,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub estimated_fps: f64,
    pub frame_counter: u64,
    pub last_update_time_ns: u64,
}

#[derive(Debug, Clone)]
pub struct FrameInfo {
    pub frame_index: u64,
    pub timestamp_ns: u64,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub dropped_frame_count: u32,
}

#[derive(Debug, Clone)]
pub struct SenderDesc {
    pub name: String,
    pub application_name: String,
    pub ring_buffer_size: u32,
    pub allow_format_fallback: bool,
}

impl Default for SenderDesc {
    fn default() -> Self {
        SenderDesc {
            name: String::new(),
            application_name: String::new(),
            ring_buffer_size: 3,
            allow_format_fallback: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReceiverDesc {
    pub name: String,
    pub application_name: String,
    pub receive_mode: ReceiveMode,
}

impl Default for ReceiverDesc {
    fn default() -> Self {
        ReceiverDesc {
            name: String::new(),
            application_name: String::new(),
            receive_mode: ReceiveMode::LatestOnly,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AcquireDesc {
    pub timeout_ms: u64,
}

impl Default for AcquireDesc {
    fn default() -> Self {
        AcquireDesc { timeout_ms: 0 }
    }
}
