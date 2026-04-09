use anyhow::Result;

pub fn run(path: Option<&str>) -> Result<()> {
    let _repo = crate::repo::init(path)?;
    let display_path = path.unwrap_or(".");
    println!("Initialized gg repository at {display_path}");
    Ok(())
}
