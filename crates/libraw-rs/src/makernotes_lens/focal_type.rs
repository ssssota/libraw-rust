#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FocalType {
    Undefined,
    Unknown(i16),
    Zoom,
    Fixed,
}
impl From<i16> for FocalType {
    fn from(value: i16) -> Self {
        match value {
            -1 => FocalType::Undefined,
            1 => FocalType::Zoom,
            2 => FocalType::Fixed,
            _ => FocalType::Unknown(value),
        }
    }
}
