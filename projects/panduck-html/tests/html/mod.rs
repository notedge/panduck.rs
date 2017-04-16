use panduck_core::{convert::parse_html, Result};
use panduck_html::HTMLRenderer;

#[test]
fn nested() -> Result<()> {
    let mut cfg = HTMLRenderer::default();
    let ast = parse_html("<p>text<i>i</i><b>b</b></p>")?;
    let out = cfg.render_pretty(&ast)?;
    println!("{}", out);
    Ok(())
}
