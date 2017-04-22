mod image;
pub use self::image::ImageConfig;

pub struct HTMLConfig {
    pub image_config: ImageConfig,
}

impl Default for HTMLConfig {
    fn default() -> Self {
        Self { image_config: Default::default() }
    }
}
