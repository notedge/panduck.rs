use super::*;

pub struct ImageConfig {
    pub lazy_loading: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self { lazy_loading: true }
    }
}

impl<T> HTMLConfig<T> {
    pub fn set_image_config(&mut self, config: ImageConfig) {
        self.image_config = config
    }
}
