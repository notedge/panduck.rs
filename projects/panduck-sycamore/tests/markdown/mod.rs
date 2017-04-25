use super::*;

#[test]
pub fn test1() -> Result<()> {
    html_fragment(include_str!("basic.md"), include_str!("basic.html"))?;
    Ok(())
}

#[test]
pub fn test2() -> Result<()> {
    html_fragment(include_str!("extensions.md"), include_str!("extensions.html"))?;
    Ok(())
}

#[test]
pub fn test3() -> Result<()> {
    html_fragment(include_str!("regression.md"), include_str!("regression.html"))?;
    Ok(())
}

#[test]
pub fn test4() -> Result<()> {
    html_fragment(include_str!("markdown-it.md"), include_str!("markdown-it.html"))?;
    Ok(())
}
