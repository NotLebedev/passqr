use printpdf::{Mm, Point, Pt, RawImage, XObjectTransform};

/// Target dots per inch for all images
pub const DPI: f32 = 300.0;
/// Target dots per mm for all images
pub const DPM: f32 = DPI / 25.4;
/// Dot size for all images
pub const MPD: f32 = 1.0 / DPM;

pub mod page {
    use super::*;

    pub const WIDTH: Mm = Mm(210.0);
    pub const HEIGHT: Mm = Mm(297.0);

    /// Equal margins on all sides of page.
    pub const MARGINS: Mm = Mm(20.0);

    pub const WORKABLE_WIDTH: Mm = Mm(WIDTH.0 - MARGINS.0 * 2.0);
    pub const WORKABLE_HEIGHT: Mm = Mm(HEIGHT.0 - MARGINS.0 * 2.0);
}

pub mod qr_single {
    //! Page with one single qr code in the middle that
    //! encodes all data together as a toml string

    use super::*;

    pub const QR_SIZE: Mm = Mm(page::WORKABLE_WIDTH.0.min(page::WORKABLE_HEIGHT.0));

    pub fn layout_qr(qr_actual_size: MmPoint) -> MmPoint {
        let dest_rect_vertex = MmPoint {
            x: page::MARGINS,
            y: page::MARGINS,
        };
        let dest_rect_size = MmPoint {
            x: page::WORKABLE_WIDTH,
            y: page::WORKABLE_HEIGHT,
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

pub mod qr_multi {
    //! Page with a grid of qr codes for individual entries.
    //!
    //! Page has [`page::MARGINS`] on all sides, but no margins
    //! between entries, because each qr code has its own quite
    //! zone embedded into image.
    //!
    //! ```
    //! +--------------------+ -
    //! |                    | page::MARGINS
    //! |  +--------------+  | -
    //! |  |    |    |    |  |
    //! |  |    |    |    |  |
    //! |  +----+----+----+  |
    //! |  |    |    |    |  |
    //! |  |    |    |    |  |
    //! |  +--------------+  |
    //! |  |    |    |    |  |
    //! |  |    |    |    |  |
    //! |  +--------------+  |
    //! |                    |
    //! +--------------------+
    //! ```
    //!
    //! Each grid cell contains a text with entry name and qr code with
    //! entry value. Each cell in grid is  [`BOX_WIDTH`] x [`BOX_HEIGHT`].
    //! On top [`TEXT_BOX_HEIGHT`] is reserved for label and the rest is
    //! allocated for qr code.
    //!
    //! ```
    //! +--------------------+ -
    //! |     Label here     | TEXT_BOX_HEIGHT
    //! +--------------------+ -
    //! |                    |
    //! |   ███   ███ █ ██   |
    //! |   █ █    ███ █ █   |
    //! |   ███  █ █ ███ █   | QR_BOX_HEIGHT
    //! |         ███ █ ██   |
    //! |   ██ ██ █ █ ██ █   |
    //! |   ██ ██ █ █ ██ █   |
    //! |                    |
    //! +--------------------+ -
    //! |     BOX_WIDTH      |
    //! ```

    use super::*;

    pub const GRID_WIDTH: u32 = 3;
    pub const GRID_HEIGHT: u32 = 4;

    pub const FONT_SIZE: Pt = Pt(10.0);
    const TEXT_BOX_HEIGHT: Mm = Mm(5.0);

    pub const BOX_WIDTH: Mm = Mm(page::WORKABLE_WIDTH.0 / GRID_WIDTH as f32);
    pub const BOX_HEIGHT: Mm = Mm(page::WORKABLE_HEIGHT.0 / GRID_HEIGHT as f32);

    pub const QR_BOX_HEIGHT: Mm = Mm(BOX_HEIGHT.0 - TEXT_BOX_HEIGHT.0);
    pub const QR_SIZE: Mm = Mm(QR_BOX_HEIGHT.0.min(BOX_WIDTH.0));

    /// Calculate place of text in given grid space (indexed from 0).
    ///
    /// Text is aligned vertically, but for horizontal alignment (centering)
    /// [`printpdf::shape`] is used. Note that [`printpdf::shape::ShapedText::get_ops`]
    /// uses TOP left corner, unlike rest of operations.
    pub fn layout_text(column: u32, row: u32) -> MmPoint {
        let x = page::MARGINS + BOX_WIDTH * column as f32;

        // Here we need to add FONT_SIZE to alignment offset because
        // text shaping expects origin to be on top left, not bottom like
        // the rest of ops.
        let text_vertical_align_offset =
            pt_to_mm(FONT_SIZE) + (TEXT_BOX_HEIGHT - pt_to_mm(FONT_SIZE)) / 2.0;
        let y =
            page::MARGINS + BOX_HEIGHT * row as f32 + QR_BOX_HEIGHT + text_vertical_align_offset;

        MmPoint { x, y }
    }

    /// Calculate place of qr code image in given grid space (indexed from 0).
    ///
    /// Centers qr code inside reserve [`BOX_WIDTH`] x [`QR_BOX_HEIGHT`] box
    pub fn layout_qr(column: u32, row: u32, qr_actual_size: MmPoint) -> MmPoint {
        let dest_rect_vertex = MmPoint {
            x: page::MARGINS + BOX_WIDTH * column as f32,
            y: page::MARGINS + BOX_HEIGHT * row as f32,
        };
        let dest_rect_size = MmPoint {
            x: BOX_WIDTH,
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

impl MmPoint {
    pub fn image_size(image: &RawImage) -> MmPoint {
        MmPoint {
            x: Mm(image.width as f32 * MPD),
            y: Mm(image.height as f32 * MPD),
        }
    }
}

impl From<MmPoint> for Point {
    fn from(value: MmPoint) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<MmPoint> for XObjectTransform {
    fn from(value: MmPoint) -> Self {
        XObjectTransform {
            translate_x: Some(value.x.into_pt()),
            translate_y: Some(value.y.into_pt()),
            ..Default::default()
        }
    }
}
