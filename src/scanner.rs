use std::path::Path;

use ignore::Walk;

pub fn scan(path: &Path) -> Walk {
    ignore::WalkBuilder::new(path)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .follow_links(false)
        .build()
}
