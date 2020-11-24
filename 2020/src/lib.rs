use serde::Serialize;

pub fn read_csv(
    path: &str,
    has_headers: Option<bool>,
) -> Result<csv::Reader<std::fs::File>, csv::Error> {
    csv::ReaderBuilder::new()
        .has_headers(has_headers.unwrap_or_else(|| false))
        .from_path(path)
}

pub fn vec_to_csv<T>(vec: Vec<T>, path: &str)
where
    T: Serialize + std::string::ToString,
{
    let mut wtr = csv::Writer::from_path(path).expect("Output file error");
    for item in vec {
        wtr.write_record(&[item.to_string()])
            .expect("Serializing error");
    }
}
