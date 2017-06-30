use wasi_notedown::exports::notedown::core::types::NotedownError;

#[derive(Default)]
pub struct ReadState {
    errors: Vec<NotedownError>
}

pub trait ReadMarkdown<Output> {
    fn into_note_down(self, state: &mut ReadState) -> Result<Output, NotedownError>;
}