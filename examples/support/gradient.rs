use nozzle::{Error, ErrorCode, Result, TextureFormat};

pub fn write_gradient(
    data: &mut [u8],
    width: u32,
    height: u32,
    stride: u32,
    format: TextureFormat,
) -> Result<()> {
    let (red, blue) = match format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8Srgb => (0, 2),
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8Srgb => (2, 0),
        _ => return Err(Error::new(ErrorCode::UnsupportedFormat)),
    };
    let width = width as usize;
    let height = height as usize;
    let stride = stride as usize;
    let invalid_layout =
        || Error::with_message(ErrorCode::InvalidArgument, "invalid gradient layout");
    let row_bytes = width.checked_mul(4).ok_or_else(invalid_layout)?;
    let required = height
        .checked_sub(1)
        .and_then(|rows| rows.checked_mul(stride))
        .and_then(|bytes| bytes.checked_add(row_bytes))
        .ok_or_else(invalid_layout)?;
    if width == 0 || stride < row_bytes || required > data.len() {
        return Err(invalid_layout());
    }
    for y in 0..height {
        for x in 0..width {
            let offset = y * stride + x * 4;
            data[offset + red] = (x / 2) as u8;
            data[offset + 1] = (y / 2) as u8;
            data[offset + blue] = 128;
            data[offset + 3] = 255;
        }
    }
    Ok(())
}
