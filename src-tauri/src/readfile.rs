use encoding_rs::UTF_16LE;
use regex::Regex;
use std::fs;
use std::path::Path;
// #[allow(dead_code)]
use serde::{Serialize, Deserialize};
use tauri::ipc::Response;
/// 将 "hh:mm:ss:ff" 格式的时间转换为 "hh:mm:ss.sss" 格式，帧率为 25fps
fn timecode_to_seconds(timecode: &str) -> Option<String> {
    let parts: Vec<&str> = timecode.split(':').collect();
    if parts.len() != 4 {
        return None;
    }
    let h = parts[0].parse::<u32>().ok()?;
    let m = parts[1].parse::<u32>().ok()?;
    let s = parts[2].parse::<u32>().ok()?;
    let f = parts[3].parse::<u32>().ok()?;
    let fps = 24.0;
    let sec = s as f64 + (f as f64 / fps);
    Some(format!("{:02}:{:02}:{:06.3}", h, m, sec))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAndFile {
    start_time: String,
    original_start_time: String,
    original_end_time: String,
    end_time: String,
    filename: String,
}
#[tauri::command]
pub fn read_file(file_path: &Path) -> Result<Vec<TimeAndFile>, String> {
   // 读取文件并处理编码问题
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
            let original_start_time= line[0].to_string();
            let original_end_time = line[1].to_string();
            let start_time = timecode_to_seconds(&original_start_time)
                .unwrap_or_else(|| original_start_time.clone());
            let end_time = timecode_to_seconds(&original_end_time)
                .unwrap_or_else(|| original_end_time.clone());
            index += 1;
            let filename = lines[index].trim().to_string();
            result.push(TimeAndFile {
                start_time,
                end_time,
                filename,
                original_start_time,
                original_end_time,
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
    println!("Getting first frame from: {:?}", path);
    let bytes = fs::read(Path::new(path)).unwrap();
    tauri::ipc::Response::new(bytes)
}