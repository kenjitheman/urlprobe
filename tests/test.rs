use urlprobe::read::{read_urls_from_csv, read_urls_from_json, read_urls_from_txt};

#[test]
fn test_read_urls_from_json_valid() {
    let json_data = r#"
    {
        "urls": [
            "https://google.com",
            "https://github.com"
        ]
    }"#;

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path();
    std::fs::write(&file_path, json_data).expect("Failed to write to the temporary file");
    let result = read_urls_from_json(&file_path.to_str().unwrap());

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}

#[test]
fn test_read_urls_from_json_invalid() {
    let json_data = "invalid JSON data";
    let result = read_urls_from_json(json_data);
    assert!(result.is_err());
}

#[test]
fn test_read_urls_from_csv_valid() {
    let csv_data = "urls\nhttps://google.com\nhttps://github.com";

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path();
    std::fs::write(&file_path, csv_data).expect("Failed to write to the temporary file");
    let result = read_urls_from_csv(&file_path.to_str().unwrap());

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}

#[test]
fn test_read_urls_from_csv_invalid() {
    let csv_data = "invalid CSV data";
    let result = read_urls_from_csv(csv_data);
    assert!(result.is_err());
}

#[test]
fn test_read_urls_from_txt_valid() {
    let txt_data = "https://google.com\nhttps://github.com";

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path();
    std::fs::write(&file_path, txt_data).expect("Failed to write to the temporary file");
    let result = read_urls_from_txt(&file_path.to_str().unwrap());

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}

#[test]
fn test_read_urls_from_txt_invalid() {
    let txt_data = "invalid text data";
    let result = read_urls_from_txt(txt_data);
    assert!(result.is_err());
}
