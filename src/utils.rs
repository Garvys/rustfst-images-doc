use std::fs::{remove_file, File};
use std::path::Path;
use std::process::{Command, Stdio};

use failure::Fallible;
use rustfst::fst_traits::SerializableFst;
use rustfst::semirings::SerializableSemiring;
use rustfst::DrawingConfig;

pub fn generate_image<P: AsRef<Path>, F: SerializableFst>(
    path_images: P,
    fst: &F,
    name: &str,
    config: &DrawingConfig,
) -> Fallible<()>
where
    F::W: SerializableSemiring,
{
    let path_images = path_images.as_ref();
    let path_dot_file = path_images.join(format!("{}.dot", name));
    fst.draw(&path_dot_file, &config)?;

    let outputs = File::create(path_images.join(format!("{}.svg", name)))?;
    Command::new("dot")
        .args(&["-Tsvg", path_dot_file.as_os_str().to_str().unwrap()])
        .stdout(Stdio::from(outputs))
        .spawn()?
        .wait_with_output()?;

    remove_file(path_dot_file)?;

    Ok(())
}
