/**
 * @params `data: &str` The string data of type T
 * @params `T` The struct with #[derive(Deserialize)]
 */
pub fn parse<'a, T: serde::de::Deserialize<'a>>(data: &'a str) -> Result<T, serde_json::Error> {
    Ok(serde_json::from_str::<T>(data)?)
}

/**
 * @params `data: &T` The data to serialize
 * @params `T` The struct with #[derive(Serialize)]
 */
pub fn stringify<T: serde::Serialize>(data: &T) -> Result<String, serde_json::Error> {
    Ok(serde_json::to_string(data)?)
}
