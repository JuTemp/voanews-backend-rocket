/// @params `filename: &str` The filename with path. eg. `"./assets/file.in"`
pub fn open(filename: &str) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(filename)?)
}

/// @params `filename: &str` The filename with path. eg. `"./assets/file.out"` <br/>
/// @params `data: &str` the data to write in. eg. `"data"`
pub fn write(filename: &str, data: &str) -> Result<(), std::io::Error> {
    Ok(std::fs::write(filename, data)?)
}

