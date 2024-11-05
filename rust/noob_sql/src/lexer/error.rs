#[derive(Debug)]
pub struct LexingError {
    pub variant: &'static str,
    pub line: usize,
    pub index: usize,
}

impl LexingError {
    pub fn new(variant: &'static str, line: usize, index: usize) -> LexingError {
        LexingError {
            variant,
            line,
            index,
        }
    }
}

impl std::error::Error for LexingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid {}Token at line {}, position {}",
            self.variant, self.line, self.index
        )
    }
}
