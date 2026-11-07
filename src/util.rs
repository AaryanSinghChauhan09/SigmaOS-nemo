//! nemo model — Cinnamon file manager.

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub current_dir: String,
    pub bookmarks: Vec<Bookmark>,
    pub files: Vec<File>,
}

impl Default for Model {
    fn default() -> Self { Self { current_dir: "/".into(), bookmarks: vec![], files: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn navigate(&mut self, path: &str) -> bool {
        if std::path::Path::new(path).exists() {
            self.current_dir = path.into();
            true
        } else { false }
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) { self.bookmarks.push(bookmark); }

    pub fn list_files(&self) -> &[File] { &self.files }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigate_bookmark() {
        let mut n = Model::new();
        assert!(n.navigate("/tmp"));
        n.add_bookmark(Bookmark { name: "tmp".into(), uri: "file:///tmp".into() });
        assert_eq!(n.bookmarks.len(), 1);
    }
}
