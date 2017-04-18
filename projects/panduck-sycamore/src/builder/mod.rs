use panduck_html::HTMLConfig;

pub struct SycamoreExtension {}

pub type SycamoreBuilder = HTMLConfig<SycamoreExtension>;

pub struct ImageConfig {
    pub lazy_loading: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self { lazy_loading: false }
    }
}
