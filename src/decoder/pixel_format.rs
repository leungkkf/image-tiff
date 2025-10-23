use crate::decoder::image::ColorTableElement;

/// Convert the decoded bytes into another pixel format.
pub(crate) trait PixelFormatConverter: std::iter::FromIterator<Self::PixelType> {
    type PixelType;

    /// Convert a colour table element into the required pixel type.
    fn convert_element(value: ColorTableElement) -> Self::PixelType;
}

impl PixelFormatConverter for Vec<(u8, u8, u8)> {
    type PixelType = (u8, u8, u8);

    fn convert_element(value: ColorTableElement) -> Self::PixelType {
        (value.r, value.g, value.b)
    }
}

impl PixelFormatConverter for Vec<u32> {
    type PixelType = u32;

    fn convert_element(value: ColorTableElement) -> Self::PixelType {
        (value.r as u32) << 16 | (value.g as u32) << 8 | value.b as u32
    }
}
