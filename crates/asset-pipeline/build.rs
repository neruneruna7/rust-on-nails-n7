use ructe::{Result, Ructe};

fn main() -> Result<()> {
    // cornucopia()?;

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("dist")?;
    statics.add_file("images")?;
    ructe.compile_templates("templates")?;

    Ok(())
}
