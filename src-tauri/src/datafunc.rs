use serde_json::{Value as JsonValue};
use std::fs;
use std::path::Path;

/// Convert data files between formats (JSON, YAML, XML, CSV, TOML)
#[tauri::command]
pub fn convert_data(
    file_path: String,
    output_dir: String,
    target_format: String,
) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    let input_ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or("No file extension")?
        .to_lowercase();

    // Read and parse input file
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let data: JsonValue = match input_ext.as_str() {
        "json" => serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?,
        "yaml" | "yml" => serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?,
        "toml" => toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML: {}", e))?,
        "xml" => parse_xml_to_json(&content)?,
        "csv" => parse_csv_to_json(&content)?,
        _ => return Err(format!("Unsupported input format: {}", input_ext)),
    };

    // Convert to target format
    let target = target_format.to_lowercase();
    let output_content = match target.as_str() {
        "json" => serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?,
        "yaml" => serde_yaml::to_string(&data)
            .map_err(|e| format!("Failed to serialize YAML: {}", e))?,
        "toml" => toml::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize TOML: {}", e))?,
        "xml" => json_to_xml(&data)?,
        "csv" => json_to_csv(&data)?,
        _ => return Err(format!("Unsupported output format: {}", target)),
    };

    // Write output file
    let output_path = format!("{}/{}.{}", output_dir, file_name, target);
    fs::write(&output_path, output_content)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    Ok(format!("Converted {} to {}", file_name, target))
}

fn parse_xml_to_json(xml: &str) -> Result<JsonValue, String> {
    use quick_xml::de::from_str;
    from_str(xml).map_err(|e| format!("Failed to parse XML: {}", e))
}

fn json_to_xml(data: &JsonValue) -> Result<String, String> {
    use quick_xml::se::to_string;
    to_string(data).map_err(|e| format!("Failed to serialize XML: {}", e))
}

fn parse_csv_to_json(csv: &str) -> Result<JsonValue, String> {
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read CSV record: {}", e))?;
        let mut obj = serde_json::Map::new();
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.clone(), JsonValue::String(field.to_string()));
            }
        }
        records.push(JsonValue::Object(obj));
    }

    Ok(JsonValue::Array(records))
}

fn json_to_csv(data: &JsonValue) -> Result<String, String> {
    match data {
        JsonValue::Array(arr) if !arr.is_empty() => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            
            // Get headers from first object
            if let Some(JsonValue::Object(first)) = arr.first() {
                let headers: Vec<&String> = first.keys().collect();
                wtr.write_record(&headers)
                    .map_err(|e| format!("Failed to write CSV headers: {}", e))?;

                // Write records
                for item in arr {
                    if let JsonValue::Object(obj) = item {
                        let row: Vec<String> = headers
                            .iter()
                            .map(|h| match obj.get(*h) {
                                Some(JsonValue::String(s)) => s.clone(),
                                Some(v) => v.to_string(),
                                None => String::new(),
                            })
                            .collect();
                        wtr.write_record(&row)
                            .map_err(|e| format!("Failed to write CSV record: {}", e))?;
                    }
                }
            }

            wtr.flush().map_err(|e| format!("Failed to flush CSV: {}", e))?;
            String::from_utf8(wtr.into_inner().map_err(|e| format!("CSV writer error: {}", e))?)
                .map_err(|e| format!("Invalid UTF-8 in CSV: {}", e))
        }
        _ => Err("CSV conversion requires a JSON array of objects".to_string()),
    }
}
