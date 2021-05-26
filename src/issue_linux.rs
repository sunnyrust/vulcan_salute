use std::error::Error;

pub(crate) fn read_issue() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("/etc/issue")?)
}
