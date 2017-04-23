#[cfg(feature = "syntect")]
mod syntect;
use self::syntect::SyntectConfig;

pub struct CodeConfig {
    #[cfg(feature = "syntect")]
    syntect_config: SyntectConfig,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self { syntect_config: Default::default() }
    }
}
