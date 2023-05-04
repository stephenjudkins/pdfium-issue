use eyre::{bail, eyre, Result};
use pdfium_render::prelude::*;

fn go() -> Result<()> {
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library()?);
    let mut doc = pdfium.create_new_pdf()?;
    let font = doc
        .fonts_mut()
        .load_true_type_from_bytes(include_bytes!("../assets/line-item-font.ttf"), true)?;
    let font_size = PdfPoints::new(18.0);

    let mut page = doc
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::new_portrait(PdfPagePaperStandardSize::A7))?;

    let mut text = PdfPageTextObject::new(&doc, "hello world", font, font_size)?;
    text.translate((page.width() - text.width()?) / 2.0, page.height() / 2.0)?;
    page.objects_mut().add_text_object(text)?;

    doc.save_to_file("out.pdf")?;
    Ok(())
}

fn main() {
    go().unwrap();
}
