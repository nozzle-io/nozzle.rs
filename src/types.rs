#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BackendType {
    Unknown = 0,
    D3D11 = 1,
    Metal = 2,
    OpenGL = 3,
    DmaBuf = 4,
}

impl BackendType {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => BackendType::D3D11,
            2 => BackendType::Metal,
            3 => BackendType::OpenGL,
            4 => BackendType::DmaBuf,
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
    Rgb8Unorm = 3,
    Rgba8Unorm = 4,
    Bgra8Unorm = 5,
    Rgba8Srgb = 6,
    Bgra8Srgb = 7,
    R16Unorm = 8,
    RG16Unorm = 9,
    Rgb16Unorm = 10,
    Rgba16Unorm = 11,
    R16Float = 12,
    RG16Float = 13,
    Rgb16Float = 14,
    Rgba16Float = 15,
    R32Float = 16,
    RG32Float = 17,
    Rgb32Float = 18,
    Rgba32Float = 19,
    R32Uint = 20,
    Rgba32Uint = 21,
    Rgb32Uint = 22,
    Depth32Float = 23,
}

impl TextureFormat {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => TextureFormat::R8Unorm,
            2 => TextureFormat::RG8Unorm,
            3 => TextureFormat::Rgb8Unorm,
            4 => TextureFormat::Rgba8Unorm,
            5 => TextureFormat::Bgra8Unorm,
            6 => TextureFormat::Rgba8Srgb,
            7 => TextureFormat::Bgra8Srgb,
            8 => TextureFormat::R16Unorm,
            9 => TextureFormat::RG16Unorm,
            10 => TextureFormat::Rgb16Unorm,
            11 => TextureFormat::Rgba16Unorm,
            12 => TextureFormat::R16Float,
            13 => TextureFormat::RG16Float,
            14 => TextureFormat::Rgb16Float,
            15 => TextureFormat::Rgba16Float,
            16 => TextureFormat::R32Float,
            17 => TextureFormat::RG32Float,
            18 => TextureFormat::Rgb32Float,
            19 => TextureFormat::Rgba32Float,
            20 => TextureFormat::R32Uint,
            21 => TextureFormat::Rgba32Uint,
            22 => TextureFormat::Rgb32Uint,
            23 => TextureFormat::Depth32Float,
            _ => TextureFormat::Unknown,
        }
    }

    pub fn bytes_per_pixel(&self) -> Option<u32> {
        match self {
            TextureFormat::R8Unorm => Some(1),
            TextureFormat::RG8Unorm => Some(2),
            TextureFormat::Rgb8Unorm => Some(3),
            TextureFormat::Rgba8Unorm => Some(4),
            TextureFormat::Bgra8Unorm => Some(4),
            TextureFormat::Rgba8Srgb => Some(4),
            TextureFormat::Bgra8Srgb => Some(4),
            TextureFormat::R16Unorm => Some(2),
            TextureFormat::RG16Unorm => Some(4),
            TextureFormat::Rgb16Unorm => Some(6),
            TextureFormat::Rgba16Unorm => Some(8),
            TextureFormat::R16Float => Some(2),
            TextureFormat::RG16Float => Some(4),
            TextureFormat::Rgb16Float => Some(6),
            TextureFormat::Rgba16Float => Some(8),
            TextureFormat::R32Float => Some(4),
            TextureFormat::RG32Float => Some(8),
            TextureFormat::Rgb32Float => Some(12),
            TextureFormat::Rgba32Float => Some(16),
            TextureFormat::R32Uint => Some(4),
            TextureFormat::Rgba32Uint => Some(16),
            TextureFormat::Rgb32Uint => Some(12),
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
            TextureFormat::Rgb8Unorm => "rgb8_unorm",
            TextureFormat::Rgba8Unorm => "rgba8_unorm",
            TextureFormat::Bgra8Unorm => "bgra8_unorm",
            TextureFormat::Rgba8Srgb => "rgba8_srgb",
            TextureFormat::Bgra8Srgb => "bgra8_srgb",
            TextureFormat::R16Unorm => "r16_unorm",
            TextureFormat::RG16Unorm => "rg16_unorm",
            TextureFormat::Rgb16Unorm => "rgb16_unorm",
            TextureFormat::Rgba16Unorm => "rgba16_unorm",
            TextureFormat::R16Float => "r16_float",
            TextureFormat::RG16Float => "rg16_float",
            TextureFormat::Rgb16Float => "rgb16_float",
            TextureFormat::Rgba16Float => "rgba16_float",
            TextureFormat::R32Float => "r32_float",
            TextureFormat::RG32Float => "rg32_float",
            TextureFormat::Rgb32Float => "rgb32_float",
            TextureFormat::Rgba32Float => "rgba32_float",
            TextureFormat::R32Uint => "r32_uint",
            TextureFormat::Rgba32Uint => "rgba32_uint",
            TextureFormat::Rgb32Uint => "rgb32_uint",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextureOrigin {
    TopLeft = 0,
    BottomLeft = 1,
}

impl TextureOrigin {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            0 => TextureOrigin::TopLeft,
            _ => TextureOrigin::BottomLeft,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TransferMode {
    Unknown = 0,
    ZeroCopySharedTexture = 1,
    GpuCopy = 2,
    CpuCopy = 3,
}

impl TransferMode {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => TransferMode::ZeroCopySharedTexture,
            2 => TransferMode::GpuCopy,
            3 => TransferMode::CpuCopy,
            _ => TransferMode::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SyncMode {
    None = 0,
    AccessGuarded = 1,
    GpuFenceBestEffort = 2,
}

impl SyncMode {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => SyncMode::AccessGuarded,
            2 => SyncMode::GpuFenceBestEffort,
            _ => SyncMode::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FormatSource {
    Unknown = 0,
    Requested = 1,
    CallerHint = 2,
    NativeObserved = 3,
}

impl FormatSource {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => FormatSource::Requested,
            2 => FormatSource::CallerHint,
            3 => FormatSource::NativeObserved,
            _ => FormatSource::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NativeFormatKind {
    Unknown = 0,
    MtlPixelFormat = 1,
    DxgiFormat = 2,
    DrmFourcc = 3,
    GlInternalFormat = 4,
}

impl NativeFormatKind {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            1 => NativeFormatKind::MtlPixelFormat,
            2 => NativeFormatKind::DxgiFormat,
            3 => NativeFormatKind::DrmFourcc,
            4 => NativeFormatKind::GlInternalFormat,
            _ => NativeFormatKind::Unknown,
        }
    }
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
    pub semantic_format: TextureFormat,
    pub estimated_fps: f64,
    pub frame_counter: u64,
    pub last_update_time_ns: u64,
    pub native_format_modifier: u64,
}

#[derive(Debug, Clone)]
pub struct FrameInfo {
    pub frame_index: u64,
    pub timestamp_ns: u64,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub semantic_format: TextureFormat,
    pub transfer_mode: TransferMode,
    pub sync_mode: SyncMode,
    pub dropped_frame_count: u32,
}

#[derive(Debug, Clone)]
pub struct ResolvedTextureFormat {
    pub storage_format: TextureFormat,
    pub semantic_format: TextureFormat,
    pub format_source: FormatSource,
    pub native_backend: BackendType,
    pub native_kind: NativeFormatKind,
    pub native_value: u32,
    pub channel_order: u32,
    pub component_type: u32,
    pub component_bits: u8,
    pub channel_count: u8,
    pub bytes_per_pixel: u8,
}

#[derive(Debug, Clone)]
pub struct TextureWrapDesc {
    pub native_texture: *mut std::ffi::c_void,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub backend: BackendType,
}

#[derive(Debug, Clone)]
pub struct SenderDesc {
    pub name: String,
    pub application_name: String,
    pub ring_buffer_size: u32,
    pub allow_format_fallback: bool,
    pub fallback_flags: u32,
    pub fallback_flags_valid: bool,
}

impl Default for SenderDesc {
    fn default() -> Self {
        SenderDesc {
            name: String::new(),
            application_name: String::new(),
            ring_buffer_size: 3,
            allow_format_fallback: true,
            fallback_flags: 3,
            fallback_flags_valid: true,
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
