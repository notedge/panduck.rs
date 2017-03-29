#[cfg(feature = "html")]
mod html;
#[cfg(feature = "jupyter")]
mod jupyter;
#[cfg(feature = "markdown")]
mod markdown;

#[test]
fn ready() {
    println!("it works!")
}
