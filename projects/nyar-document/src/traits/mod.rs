use std::sync::Arc;

/// A element that had a single page
pub trait PagedElement {
    fn new<S: ToString>(name: S) -> Arc<Self>;
    fn set_summary<S: ToString>(&mut self, summary: S);
}