use std::path::PathBuf;

pub(crate) fn find_logseq_page(name: &str) -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    let logseq_dir = home_dir.join("logseq-repo").join("pages");
    let page_path = logseq_dir.join(format!("{}.md", name));
    if page_path.exists() {
        Some(page_path)
    } else {
        None
    }
}
