#[derive(Debug)]
pub enum TextFormat {
    AsSingleLine,
    AsMultiLine,
    AsRow,
    AsHeader,
}

pub trait Text {
    fn text(&self, format: TextFormat) -> Vec<String>;
}
