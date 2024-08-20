use git2::Repository;

pub struct Plugin {
    name: String,
    version: String,
    categories: Vec<String>,
    git_repo: String,
}

pub fn available_plugins() -> Result<Vec<String>, anyhow::Error> {
    let repo = Repository::open("https://github.com/rhaskia/preTTYplugins.git")?;

    let mut folders = Vec::new();
    for entry in repo.tree(repo.head()?.target().unwrap())? {
        let entry = entry?;
        if entry.kind() == git2::ObjectType::Tree {
            let path = entry.name().unwrap().to_owned();
            folders.push(path);
        }
    }

    Ok(folders)
}
