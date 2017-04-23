mod code;
mod image;

pub use self::{code::CodeConfig, image::ImageConfig};

pub struct HTMLConfig {
    pub image_config: ImageConfig,
    pub code_config: CodeConfig,
    pub math_config: MathConfig,
}

pub struct MathConfig {}

impl Default for MathConfig {
    fn default() -> Self {
        Self {}
    }
}

impl Default for HTMLConfig {
    fn default() -> Self {
        Self { image_config: Default::default(), code_config: Default::default(), math_config: Default::default() }
    }
}
