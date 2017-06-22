/// The result type for document operations
pub type DocumentResult<T> = std::result::Result<T, DocumentError>;

/// D
#[derive(Debug, Clone)]
pub struct DocumentError {
    kind: Box<DocumentErrorKind>,
}

///
#[derive(Debug, Clone)]
pub enum DocumentErrorKind {
    ///
    IoError {
        ///
        message: String,
        ///
        file: String,
    },
}

impl DocumentError {
    pub fn as_kind(&self) -> &DocumentErrorKind {
        &self.kind
    }

    /// Create a new io error
    pub fn io_error<S: ToString>(message: S) -> Self {
        Self { kind: Box::new(DocumentErrorKind::IoError { message: message.to_string(), file: "".to_string() }) }
    }
}
