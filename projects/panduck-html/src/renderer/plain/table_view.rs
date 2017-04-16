use super::*;

impl PlainHTML for TableView {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}

#[allow(dead_code)]
pub fn build_th(input: &str, e: u8) -> String {
    match e {
        1 => format!(r#"<th align="left">{}</th>"#, input),
        2 => format!(r#"<th align="right">{}</th>"#, input),
        3 => format!(r#"<th align="center">{}</th>"#, input),
        _ => format!("<th>{}</th>", input),
    }
}

#[allow(dead_code)]
pub fn build_td(input: &str, e: u8) -> String {
    match e {
        1 => format!(r#"<td align="left">{}</td>"#, input),
        2 => format!(r#"<td align="right">{}</td>"#, input),
        3 => format!(r#"<td align="center">{}</td>"#, input),
        _ => format!("<td>{}</td>", input),
    }
}
