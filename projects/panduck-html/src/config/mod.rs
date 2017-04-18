mod image;
pub use self::image::ImageConfig;
use std::ops::Deref;

pub struct HTMLConfig<E> {
    pub image_config: ImageConfig,
    pub extension: E,
}

impl<T> Default for HTMLConfig<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { image_config: Default::default(), extension: Default::default() }
    }
}

impl<T> Deref for HTMLConfig<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.extension
    }
}
