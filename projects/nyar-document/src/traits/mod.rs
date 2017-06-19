use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A element that had a single page
pub trait PagedElement {
    fn new<S: ToString>(name: S) -> Self;
    fn get_name(&self) -> &str;
    fn get_namespace(&self) -> &[String];
    fn set_namespace(&mut self, namepath: Vec<String>);
    fn get_summary(&self) -> &str;
    fn set_summary<S: ToString>(&mut self, summary: S);
    fn html_path<P: AsRef<Path>>(&self, root: P) -> PathBuf {
        let mut path = root.as_ref().to_path_buf();
        for item in self.get_namespace() {
            path.push(item);
        }
        path.push(format!("{}.html", self.get_name()));
        path
    }
}