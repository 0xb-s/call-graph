
pub mod ast_extractor;

use crate::errors::AppError;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use self::ast_extractor::{extract_ast_info, FileAst};

pub fn parse_source_files(dir_path: &str) -> Result<Vec<FileAst>, AppError> {
    let path = Path::new(dir_path);

    if !path.exists() {
        return Err(AppError::Generic(format!("Input directory does not exist: {}", dir_path)));
    }

    let mut asts = Vec::new();

    for entry in WalkDir::new(path) {
  let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "rs") {
            let ast_info = parse_single_file(entry_path)?;
            asts.push(ast_info);
        }
    }

    Ok(asts)
}


fn parse_single_file(file_path: &Path) -> Result<FileAst, AppError> {
    let file_content = std::fs::read_to_string(file_path)?;
    let ast_info = extract_ast_info(file_path.to_path_buf(), &file_content)?;
    Ok(ast_info)
}
