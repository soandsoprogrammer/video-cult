use encoding_rs::UTF_16LE;
use regex::Regex;
use std::fs;
use std::path::Path;
// #[allow(dead_code)]
use serde::{Serialize, Deserialize};
use tauri::ipc::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAndFile {
    start_time: String,
    end_time: String,
    filename: String,
}
#[tauri::command]
pub fn read_file(path: &Path) -> Result<Vec<TimeAndFile>, String> {
   // 读取文件并处理编码问题
    let file_path = path;
    println!("Reading file: {:?}", file_path);
    let bytes = fs::read(Path::new(file_path)).unwrap();
    let (cow, _, _) = UTF_16LE.decode(&bytes);
    let content = cow.into_owned();
    let re = Regex::new(r"\d{2}:\d{2}:\d{2}:\d{2}").unwrap();
    let mut result: Vec<TimeAndFile> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let mut index = 0;
    while index < total_lines {
        let trimmed = lines[index].trim();
        if re.is_match(trimmed) {
            let line = trimmed.split_whitespace().collect::<Vec<&str>>();
            let start_time = line[0].to_string();
            let end_time = line[1].to_string();
            index += 1;
            let filename = lines[index].trim().to_string();
            result.push(TimeAndFile {
                start_time,
                end_time,
                filename,
            });
        }
        index += 1;
    }
    if result.is_empty() {
        return Err("No valid data found in the file".to_string());
    } else {
        println!("Parsed {} entries from the file", result.len());
    }
    Ok(result)
}


#[tauri::command]
pub fn get_first_frame(path: &Path) -> Response {
    let file_path = path;
    let bytes = fs::read(Path::new(file_path)).unwrap();
    tauri::ipc::Response::new(bytes)
}