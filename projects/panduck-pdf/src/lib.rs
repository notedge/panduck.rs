use fluent_bundle::FluentResource;
use fluent_templates::static_loader;

static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
        // Optional: A fluent resource that is shared with every locale.
        core_locales: "locales/core.ftl",
        // Optional: A function that is run over each fluent bundle.
        customise: |bundle| {
            // Since this will be called for each locale bundle and
            // `FluentResource`s need to be either `&'static` or behind an
            // `Arc` it's recommended you use lazily initialised
            // static variables.
            static CRATE_VERSION_FTL: Lazy<FluentResource> = Lazy::new(|| {
                let ftl_string = String::from(
                    concat!("-crate-version = {}", env!("CARGO_PKG_VERSION"))
                );

                FluentResource::try_new(ftl_string).unwrap()
            });

            bundle.add_resource(&CRATE_VERSION_FTL);
        }
    };
}

