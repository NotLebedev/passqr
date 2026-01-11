use ::image::Luma;
use printpdf::{
    BuiltinFont, FontId, Mm, Op, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, RawImage,
    RawImageData, TextAlign, TextShapingOptions,
};
use qrcode::QrCode;

use crate::layout::MmPoint;

mod input;
mod layout;

fn main() {
    let data = input::load_input();
    let toml_string = toml::to_string(&data).expect("Failed to serialize data");

    let mut doc = PdfDocument::new("passqr");

    // Workaround for https://github.com/fschutt/printpdf/issues/238
    let font_bytes = BuiltinFont::Helvetica.get_subset_font().bytes;
    let font_id = doc.add_font(&ParsedFont::from_bytes(&font_bytes, 0, &mut Vec::new()).unwrap());

    let mut pages = Vec::new();

    let mut data_iter = data.iter().peekable();

    while data_iter.peek().is_some() {
        pages.push(page_8_qrs(&mut doc, &mut data_iter, &font_id));
    }

    pages.push(full_qr_page(&mut doc, &toml_string));

    let pdf_bytes = doc
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new());
    std::fs::write("./output.pdf", pdf_bytes).unwrap();

    println!("Generated output.pdf");
}

fn page_8_qrs<'el>(
    doc: &mut PdfDocument,
    data: &mut impl Iterator<Item = (&'el String, &'el String)>,
    font: &FontId,
) -> PdfPage {
    let mut ops = Vec::new();

    // Iterating over rows first, column second, reversed column order
    // to provide left to right, top to bottom order on partially
    // filled pages
    'all: for j in (0..layout::qr_multi::GRID_HEIGHT).rev() {
        for i in 0..layout::qr_multi::GRID_WIDTH {
            let Some((key, value)) = data.next() else {
                break 'all;
            };

            let qr_image = generate_qr_code(value, layout::qr_multi::QR_SIZE);

            let qr_actual_size = MmPoint::image_size(&qr_image);
            let qr_image_layout = layout::qr_multi::layout_qr(i, j, qr_actual_size);

            let image_id = doc.add_image(&qr_image);
            ops.push(Op::UseXobject {
                id: image_id,
                transform: qr_image_layout.into(),
            });

            let text_layout = layout::qr_multi::layout_text(i, j);

            let text_shape = TextShapingOptions {
                font_size: layout::qr_multi::FONT_SIZE,
                max_width: Some(layout::qr_multi::BOX_WIDTH.into_pt()),
                align: TextAlign::Center,
                ..Default::default()
            };

            let text = doc.shape_text(&key, font, &text_shape).unwrap();
            ops.extend(text.get_ops(text_layout.into()));
        }
    }

    PdfPage::new(layout::page::WIDTH, layout::page::HEIGHT, ops)
}

fn full_qr_page(doc: &mut PdfDocument, data: &str) -> PdfPage {
    let mut ops = Vec::new();

    let qr_image = generate_qr_code(data, layout::qr_single::QR_SIZE);
    let image_id = doc.add_image(&qr_image);

    let qr_actual_size = MmPoint::image_size(&qr_image);
    let layout = layout::qr_single::layout_qr(qr_actual_size);
    ops.push(Op::UseXobject {
        id: image_id,
        transform: layout.into(),
    });

    PdfPage::new(layout::page::WIDTH, layout::page::HEIGHT, ops)
}

fn generate_qr_code(content: &str, size: Mm) -> RawImage {
    let code = QrCode::with_error_correction_level(content, qrcode::EcLevel::L)
        .expect("Failed to create QR code");

    let size = (size.0 * layout::DPM) as u32;
    let image = code.render::<Luma<u8>>().max_dimensions(size, size).build();
    let real_width = image.width();
    let real_height = image.height();
    RawImage {
        pixels: RawImageData::U8(image.into_vec()),
        width: real_width as usize,
        height: real_height as usize,
        data_format: printpdf::RawImageFormat::R8,
        tag: Vec::new(),
    }
}
