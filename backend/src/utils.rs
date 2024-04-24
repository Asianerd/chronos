pub fn get_date(t: u128) -> u128 {
    // input: epoch unix
    // returns date in epoch-ish format
    // 0 -> 1 jan 1970
    // 1 -> 2 jan 1970

    ((t as f64) / (86400f64)).floor() as u128
}

pub fn parse_response(data: Option<String>) -> String {
    format!(r#"{{
    "type":"{}",
    "data":"{}"
}}"#, if data.is_some() { "success" } else { "fail" }, if data.is_some() { 
        urlencoding::encode(
            data.unwrap()
                // .replace("\"", "\\\"")
                .as_str()
        ).to_string()
    } else { "".to_string() }).to_string()
}
