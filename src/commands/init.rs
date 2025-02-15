use std::{
    env,
    fs::{self},
    io,
};

pub fn init() -> io::Result<()> {
    let dir = env::current_dir()?;
    let repo_path = dir.join(".rit");
    if repo_path.exists() {
        // TODO real git will recreate a new repository
        println!("You already got a mit repository");
        return Ok(());
    }
    println!("Current directory : {}", dir.display());
    fs::create_dir_all(repo_path.join("objects"))?;
    fs::create_dir_all(repo_path.join("refs"))?;
    fs::write(repo_path.join("HEAD"), "ref: refs/heads/main\n")?;
    println!("Initialized empty Rit repository in {}",dir.display());
    Ok(())
}
