#[cfg(feature = "syntect")]
mod syntect;
#[cfg(feature = "syntect")]
use self::syntect::SyntectConfig;

pub struct CodeConfig {
    #[cfg(feature = "syntect")]
    pub syntect_config: SyntectConfig,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "syntect")]
            syntect_config: Default::default(),
        }
    }
}
