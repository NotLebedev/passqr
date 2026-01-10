use printpdf::Mm;

pub const DPI: f32 = 300.0;
pub const DPM: f32 = DPI / 25.4;

pub const BIG_QR_SIZE: Mm = Mm(130.0);

pub mod page {
    use super::*;

    pub const WIDTH: Mm = Mm(210.0);
    pub const HEIGHT: Mm = Mm(297.0);
}
