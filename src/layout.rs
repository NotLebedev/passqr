use printpdf::{Mm, Pt};

pub const DPI: f32 = 300.0;
pub const DPM: f32 = DPI / 25.4;
pub const MPD: f32 = 1.0 / DPM;

pub const BIG_QR_SIZE: Mm = Mm(130.0);

pub mod page {
    use super::*;

    pub const WIDTH: Mm = Mm(210.0);
    pub const HEIGHT: Mm = Mm(297.0);
}

pub mod qr_multi {
    //! Page with 4x2 grid of qr codes for individual entries.
    //!
    //! Each entry contains a text with entry name and
    //! qr code with entry value. Page has [`PAGE_MARGINS`] on
    //! all sides, but no margins between entries, because each
    //! qr code has its own quite zone embedded into image.

    use super::*;

    /// Equal margins on all sides of page.
    const PAGE_MARGINS: Mm = Mm(20.0);

    const WORKABLE_WIDTH: Mm = Mm(page::WIDTH.0 - PAGE_MARGINS.0 * 2.0);
    const WORKABLE_HEIGHT: Mm = Mm(page::HEIGHT.0 - PAGE_MARGINS.0 * 2.0);

    pub const FONT_SIZE: Pt = Pt(10.0);
    const TEXT_BOX_HEIGHT: Mm = Mm(5.0);

    pub const QR_BOX_WIDTH: Mm = Mm(WORKABLE_WIDTH.0 / 2.0);
    pub const QR_BOX_HEIGHT: Mm = Mm(WORKABLE_HEIGHT.0 / 4.0 - TEXT_BOX_HEIGHT.0);
    pub const QR_SIZE: Mm = Mm(QR_BOX_HEIGHT.0.min(QR_BOX_WIDTH.0));

    /// Calculate place of text in given grid space (indexed from 0).
    ///
    /// Text is aligned vertically, but for horizontal alignment (centering)
    /// [`printpdf::shape`] is used.
    pub fn layout_text(column: u32, row: u32) -> MmPoint {
        let x = PAGE_MARGINS + WORKABLE_WIDTH / 2.0 * column as f32;

        let text_vertical_align_offset = (TEXT_BOX_HEIGHT - pt_to_mm(FONT_SIZE)) / 2.0;
        let y = PAGE_MARGINS
            + WORKABLE_HEIGHT / 4.0 * row as f32
            + QR_BOX_HEIGHT
            + text_vertical_align_offset;

        MmPoint { x, y }
    }

    /// Calculate place of qr code image in given grid space (indexed from 0).
    ///
    /// Centers qr code inside reserve [`QR_BOX_WIDTH`] x [`QR_BOX_HEIGHT`] box
    pub fn layout_qr(column: u32, row: u32, qr_actual_size: MmPoint) -> MmPoint {
        let dest_rect_vertex = MmPoint {
            x: PAGE_MARGINS + WORKABLE_WIDTH / 2.0 * column as f32,
            y: PAGE_MARGINS + WORKABLE_HEIGHT / 4.0 * row as f32,
        };
        let dest_rect_size = MmPoint {
            x: QR_BOX_WIDTH,
            y: QR_BOX_HEIGHT,
        };

        center_in(
            MmRect {
                vertex: dest_rect_vertex,
                size: dest_rect_size,
            },
            qr_actual_size,
        )
    }
}

pub struct MmPoint {
    pub x: Mm,
    pub y: Mm,
}

pub struct MmRect {
    pub vertex: MmPoint,
    pub size: MmPoint,
}

const fn pt_to_mm(pt: Pt) -> Mm {
    Mm(pt.0 * 0.35277)
}

/// Calculate bottom-left corner position needed
/// to place rectangle of `target_size` centered
/// inside `destination`
fn center_in(destination: MmRect, target_size: MmPoint) -> MmPoint {
    MmPoint {
        x: destination.vertex.x + (destination.size.x - target_size.x) / 2.0,
        y: destination.vertex.y + (destination.size.y - target_size.y) / 2.0,
    }
}
