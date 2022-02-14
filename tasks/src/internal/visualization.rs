#[derive(Debug)]
pub enum TextFormat {
    AsSingleLine,
    AsMultiLine,
    AsRow,
    AsHeader,
}

pub trait Text {
    fn text(&self, format: &TextFormat) -> Vec<String>;
}

macro_rules! impl_text {
    ($($t:ty),*) => {
        $(
            impl Text for $t {
                fn text(&self, _: &TextFormat) -> Vec<String> {
                    vec![self.to_string()]
                }
            }
        )*
    }
}

impl_text!(
    bool,
    char,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    String,
    &str,
    serde_json::Value
);

impl<T> Text for Option<T>
where
    T: Text,
{
    fn text(&self, format: &TextFormat) -> Vec<String> {
        match self {
            None => Vec::new(),
            Some(t) => t.text(format),
        }
    }
}

impl Text for () {
    fn text(&self, _: &TextFormat) -> Vec<String> {
        vec![String::new()]
    }
}

impl<A> Text for (A,)
where
    A: Text,
{
    fn text(&self, format: &TextFormat) -> Vec<String> {
        match format {
            TextFormat::AsSingleLine => self.0.text(&TextFormat::AsSingleLine),
            TextFormat::AsMultiLine => self.0.text(format),
            TextFormat::AsRow => self.0.text(&TextFormat::AsSingleLine),
            TextFormat::AsHeader => vec![String::new()],
        }
    }
}
