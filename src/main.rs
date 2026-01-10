use ::image::Luma;
use printpdf::{
    Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, RawImage, RawImageData, XObjectTransform,
};
use qrcode::QrCode;

mod input;
mod layout;

fn main() {
    let data = input::load_input();
    let toml_string = toml::to_string(&data).expect("Failed to serialize data");

    let mut doc = PdfDocument::new("passqr");
    let mut pages = Vec::new();

    pages.push(full_qr_page(&mut doc, &toml_string));

    let pdf_bytes = doc
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new());
    std::fs::write("./output.pdf", pdf_bytes).unwrap();

    println!("Generated output.pdf");
}

fn full_qr_page(doc: &mut PdfDocument, data: &str) -> PdfPage {
    let mut ops = Vec::new();

    let qr_image = generate_qr_code(data, layout::BIG_QR_SIZE);
    let image_id = doc.add_image(&qr_image);
    ops.push(Op::UseXobject {
        id: image_id,
        transform: XObjectTransform {
            translate_x: Some(((layout::page::WIDTH - layout::BIG_QR_SIZE) / 2.0).into_pt()),
            translate_y: Some(((layout::page::HEIGHT - layout::BIG_QR_SIZE) / 2.0).into_pt()),
            ..Default::default()
        },
    });

    PdfPage::new(layout::page::WIDTH, layout::page::HEIGHT, ops)
}

fn generate_qr_code(content: &str, size: Mm) -> RawImage {
    let code = QrCode::with_error_correction_level(content, qrcode::EcLevel::L)
        .expect("Failed to create QR code");

    let size = (size.0 * layout::DPM) as u32;
    let image = code.render::<Luma<u8>>().min_dimensions(size, size).build();
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
