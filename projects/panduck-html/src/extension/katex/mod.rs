use katex::{error::Error, render_with_opts, OutputType};
use notedown_ast::{NoteError, Result};

pub fn katex_inline(input: &str) -> Result<String> {
    let mut cfg = katex::Opts::default();
    cfg.set_output_type(OutputType::Html);
    handler(render_with_opts(input, &cfg))
}

pub fn katex_display(input: &str) -> Result<String> {
    let mut cfg = katex::Opts::default();
    cfg.set_output_type(OutputType::Html);
    cfg.set_display_mode(true);
    handler(render_with_opts(input, &cfg))
}

fn handler<T>(out: katex::error::Result<T, Error>) -> Result<T> {
    match out {
        Ok(o) => Ok(o),
        Err(e) => Err(NoteError::runtime_error(format!("{:?}", e))),
    }
}
