use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// A element that had a single page
pub trait PagedElement {
    /// Create a new element
    fn new<S: ToString>(name: S) -> Self;
    /// Get the kind of the element
    fn get_kind(&self, plural: bool) -> &'static str;
    /// Get the name of the element
    fn get_name(&self) -> &str;
    /// Get the namespace of the element
    fn get_namespace(&self) -> &[String];
    /// Set the namespace of the element
    fn set_namespace(&mut self, namepath: Vec<String>);
    /// Get the name path of the element
    fn get_name_path(&self) -> String {
        self.get_namespace().join("::") + "::" + self.get_name()
    }

    /// Get the summary of the element
    fn get_summary(&self) -> &str;
    /// Set the summary of the element
    fn set_summary<S: ToString>(&mut self, summary: S);
    /// Get the href of the element
    fn href_in_module(&self) -> String {
        format!("./{}.html", self.get_name())
    }
    /// Get the href class of the element
    fn href_class(&self) -> String {
        format!("token-{}", self.get_kind(false))
    }
    /// Get the href of the element
    fn href_head(&self) -> String {
        let mut head = self.get_kind(true).to_string();
        head.push(' ');
        let heads = self.get_namespace().len().saturating_sub(1);
        for (i, name) in self.get_namespace().iter().enumerate() {
            head.push_str(&format!("<a href=\"{}index.html\">{}</a>::<wbr>", "../".repeat(heads.saturating_sub(i)), name));
        }
        head.push_str(&format!("<a class=\"{}\" href=\"#Self\">{}</a>", self.href_class(), self.get_name()));
        head
    }
    /// Get the html path of the element
    fn html_directory<P: AsRef<Path>>(&self, root: P) -> PathBuf {
        let mut path = root.as_ref().to_path_buf();
        for item in self.get_namespace() {
            path.push(item);
        }
        path
    }
    /// Get the html path of the element
    fn html_file<P: AsRef<Path>>(&self, root: P) -> std::io::Result<File> {
        let mut path = self.html_directory(root);
        std::fs::create_dir_all(&path)?;
        path.push(format!("{}.html", self.get_name()));
        File::create(&path)
    }
}
