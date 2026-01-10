use printpdf::{Mm, Pt};

pub const DPI: f32 = 300.0;
pub const DPM: f32 = DPI / 25.4;

pub const BIG_QR_SIZE: Mm = Mm(130.0);

pub mod page {
    use super::*;

    pub const WIDTH: Mm = Mm(210.0);
    pub const HEIGHT: Mm = Mm(297.0);
}

pub mod qr_multi {
    use super::*;

    pub const FONT_SIZE: Pt = Pt(10.0);
    pub const QR_SIZE: Mm = Mm(50.0);
    pub const OFFSET_X: Mm = Mm(page::WIDTH.0 / 3.0);
    pub const OFFSET_Y: Mm = Mm(page::HEIGHT.0 / 5.0);
}
