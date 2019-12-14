use exitfailure::ExitFailure;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use failure::format_err;

pub fn find(
    pattern: &str,
    path: &PathBuf,
    writer: Rc<RefCell<impl Write>>,
) -> Result<(), ExitFailure> {
    for entry in path.read_dir()? {
        let entry = entry?.path();
        if entry.is_file()
            && entry
                .file_stem()
                .ok_or(format_err!("could not read file name"))?
                .to_str()
                .ok_or(format_err!("could not read file name"))?
                .contains(&pattern)
        {
            writeln!(
                writer.borrow_mut(),
                "{}",
                entry.to_str().ok_or(format_err!("could not read path"))?
            )?;
        } else if entry.is_dir() {
            find(pattern, &entry, writer.clone())?;
        }
    }
    Ok(())
}
