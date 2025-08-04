use std::path::{Path, PathBuf};
use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

pub struct SearchQuery {
    pub pattern: String,
    pub is_regex: bool,
    pub case_sensitive: bool,
    pub search_in_content: bool,
    pub file_types: Vec<String>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
    pub modified_after: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_before: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            is_regex: false,
            case_sensitive: false,
            search_in_content: false,
            file_types: Vec::new(),
            size_min: None,
            size_max: None,
            modified_after: None,
            modified_before: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub file_name: String,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub match_context: Option<String>,
}

pub struct FileSearcher {
    query: SearchQuery,
    results: Vec<SearchResult>,
}

impl FileSearcher {
    pub fn new(query: SearchQuery) -> Self {
        Self {
            query,
            results: Vec::new(),
        }
    }

    pub async fn search(&mut self, root_path: &Path) -> Result<Vec<SearchResult>> {
        self.results.clear();

        let regex = if self.query.is_regex {
            Some(if self.query.case_sensitive {
                Regex::new(&self.query.pattern)?
            } else {
                Regex::new(&format!("(?i){}", self.query.pattern))?
            })
        } else {
            None
        };

        for entry in WalkDir::new(root_path).follow_links(false) {
            let entry = entry?;
            let path = entry.path();

            if self.matches_criteria(path, &regex)? {
                let metadata = entry.metadata()?;
                let result = SearchResult {
                    path: path.to_path_buf(),
                    file_name: path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    size: metadata.len(),
                    modified: chrono::DateTime::from(metadata.modified()?),
                    match_context: if self.query.search_in_content && path.is_file() {
                        self.search_in_file_content(path)?
                    } else {
                        None
                    },
                };
                self.results.push(result);
            }
        }

        Ok(self.results.clone())
    }

    fn matches_criteria(&self, path: &Path, regex: &Option<Regex>) -> Result<bool> {
        let file_name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy();

        // Check file name pattern
        let name_matches = if let Some(ref regex) = regex {
            regex.is_match(&file_name)
        } else {
            let pattern = if self.query.case_sensitive {
                self.query.pattern.clone()
            } else {
                self.query.pattern.to_lowercase()
            };
            
            let name_to_check = if self.query.case_sensitive {
                file_name.to_string()
            } else {
                file_name.to_lowercase()
            };
            
            name_to_check.contains(&pattern)
        };

        if !name_matches {
            return Ok(false);
        }

        // Check file type
        if !self.query.file_types.is_empty() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if !self.query.file_types.contains(&ext) {
                    return Ok(false);
                }
            } else if !self.query.file_types.contains(&"".to_string()) {
                return Ok(false);
            }
        }

        // Check file size
        if path.is_file() {
            let metadata = std::fs::metadata(path)?;
            let size = metadata.len();

            if let Some(min_size) = self.query.size_min {
                if size < min_size {
                    return Ok(false);
                }
            }

            if let Some(max_size) = self.query.size_max {
                if size > max_size {
                    return Ok(false);
                }
            }

            // Check modification time
            let modified: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(metadata.modified()?);

            if let Some(after) = self.query.modified_after {
                if modified < after {
                    return Ok(false);
                }
            }

            if let Some(before) = self.query.modified_before {
                if modified > before {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn search_in_file_content(&self, path: &Path) -> Result<Option<String>> {
        // Only search in text files
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if !Self::is_text_file(&ext) {
                return Ok(None);
            }
        }

        match std::fs::read_to_string(path) {
            Ok(content) => {
                let search_content = if self.query.case_sensitive {
                    content.clone()
                } else {
                    content.to_lowercase()
                };

                let pattern = if self.query.case_sensitive {
                    self.query.pattern.clone()
                } else {
                    self.query.pattern.to_lowercase()
                };

                if search_content.contains(&pattern) {
                    // Find the first occurrence and return some context
                    if let Some(pos) = search_content.find(&pattern) {
                        let start = pos.saturating_sub(50);
                        let end = std::cmp::min(pos + pattern.len() + 50, content.len());
                        let context = &content[start..end];
                        return Ok(Some(context.to_string()));
                    }
                }
            }
            Err(_) => {
                // File couldn't be read as text, skip
            }
        }

        Ok(None)
    }

    fn is_text_file(extension: &str) -> bool {
        matches!(extension,
            "txt" | "md" | "rst" | "log" | "cfg" | "conf" | "ini" |
            "json" | "xml" | "yaml" | "yml" | "toml" |
            "rs" | "py" | "js" | "ts" | "html" | "css" | "scss" |
            "c" | "cpp" | "h" | "hpp" | "java" | "go" | "php" |
            "rb" | "pl" | "sh" | "bash" | "ps1" | "bat" | "cmd"
        )
    }

    pub fn get_results(&self) -> &Vec<SearchResult> {
        &self.results
    }
}
