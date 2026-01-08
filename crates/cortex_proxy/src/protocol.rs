use serde_json::json;

pub fn frame_message(pattern: &str, data: &str, id: &str) -> String {
    let payload = json!({
        "pattern": pattern,
        "data": data,
        "id": id
    });
    let json_str = payload.to_string();
    format!("{}#{}", json_str.len(), json_str)
}

pub fn parse_response(raw: &str) -> String {
    if let Some(idx) = raw.find('#') {
        raw[idx+1..].to_string()
    } else {
        raw.to_string()
    }
}
