use std::error::Error;

use dart_kernel::component::ComponentFile;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = ComponentFile::open("hello_world.dill")?;
    println!("{:#?}", file.libraries()?);
    Ok(())
}
