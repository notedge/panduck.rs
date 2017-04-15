use crate::ToNotedown;
use docx_rs::*;
use notedown_ast::ASTNode;

pub fn hello() -> Result<(), DocxError> {
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello"))).build().pack(file)?;
    Ok(())
}

impl ToNotedown for Docx {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}

impl ToNotedown for Paragraph {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}

impl ToNotedown for ParagraphChild {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}
