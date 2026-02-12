use vectorcade_shared::font::{FontStyleId, VectorFont};

pub struct FontRegistry {
    fonts: Vec<Box<dyn VectorFont + Send + Sync>>,
}

impl FontRegistry {
    pub fn new() -> Self { Self { fonts: Vec::new() } }

    pub fn register<F: VectorFont + Send + Sync + 'static>(&mut self, f: F) {
        self.fonts.push(Box::new(f));
    }

    pub fn get(&self, style: FontStyleId) -> Option<&(dyn VectorFont + Send + Sync)> {
        self.fonts.iter().find(|f| f.style_id() == style).map(|b| b.as_ref())
    }
}
