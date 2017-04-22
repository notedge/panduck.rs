use super::*;

#[test]
pub fn test1() -> Result<()> {
    html_fragment(include_str!("test1.md"), include_str!("test1.html"))?;
    Ok(())
}
