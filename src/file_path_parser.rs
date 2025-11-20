#[derive(Debug)]
pub struct ParsedFilePath {
    pub file_path: String,
    pub line_numbers: Vec<i32>,
}

impl ParsedFilePath {
    pub fn from_args(file_path: &str, line_numbers: Vec<i32>) -> Result<Self, String> {
        if file_path.is_empty() {
            return Err("Empty file path".to_string());
        }

        // Validate file path format
        Self::validate_file_path(file_path)?;

        // Validate line numbers
        for line_num in &line_numbers {
            if *line_num <= 0 {
                return Err(format!(
                    "Line numbers must be positive integers, got: {}",
                    line_num
                ));
            }
        }

        Ok(ParsedFilePath {
            file_path: file_path.to_string(),
            line_numbers,
        })
    }

    fn validate_file_path(path: &str) -> Result<(), String> {
        // Block dangerous characters first
        if path.contains('\0') || path.contains('\n') {
            return Err("Invalid characters in file path".to_string());
        }

        // Prevent path traversal
        if path.contains("../") {
            return Err("Path traversal not allowed".to_string());
        }

        // Remove optional "./" prefix for validation
        let clean_path = path.strip_prefix("./").unwrap_or(path);

        // Must end with _spec.rb and have content before it
        if !clean_path.ends_with("_spec.rb") {
            return Err("File must be an RSpec test file (*_spec.rb)".to_string());
        }

        if clean_path == "_spec.rb" {
            return Err("Invalid file path format".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_args_without_line_numbers() {
        let parsed = ParsedFilePath::from_args("spec/models/user_spec.rb", vec![]).unwrap();
        assert_eq!(parsed.file_path, "spec/models/user_spec.rb");
        assert!(parsed.line_numbers.is_empty());
    }

    #[test]
    fn test_from_args_with_line_numbers() {
        let parsed = ParsedFilePath::from_args("spec/models/user_spec.rb", vec![37, 87]).unwrap();
        assert_eq!(parsed.file_path, "spec/models/user_spec.rb");
        assert_eq!(parsed.line_numbers, vec![37, 87]);
    }

    #[test]
    fn test_from_args_with_zero_line_number() {
        let result = ParsedFilePath::from_args("spec/models/user_spec.rb", vec![0]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Line numbers must be positive integers, got: 0"
        );
    }

    #[test]
    fn test_from_args_with_negative_line_number() {
        let result = ParsedFilePath::from_args("spec/models/user_spec.rb", vec![-5]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Line numbers must be positive integers, got: -5"
        );
    }

    #[test]
    fn test_from_args_empty_file_path() {
        let result = ParsedFilePath::from_args("", vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty file path");
    }

    #[test]
    fn test_validate_rspec_file_extension() {
        let result = ParsedFilePath::from_args("spec/models/user.rb", vec![]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File must be an RSpec test file (*_spec.rb)"
        );
    }

    #[test]
    fn test_validate_rspec_file_with_optional_prefix() {
        let parsed = ParsedFilePath::from_args("./spec/models/user_spec.rb", vec![]).unwrap();
        assert_eq!(parsed.file_path, "./spec/models/user_spec.rb");
        assert!(parsed.line_numbers.is_empty());
    }

    #[test]
    fn test_validate_path_traversal_prevention() {
        let result = ParsedFilePath::from_args("../spec/user_spec.rb", vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Path traversal not allowed");
    }

    #[test]
    fn test_validate_only_spec_extension() {
        let result = ParsedFilePath::from_args("_spec.rb", vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid file path format");
    }

    #[test]
    fn test_validate_dangerous_characters() {
        let result = ParsedFilePath::from_args("spec/user_spec.rb\0", vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid characters in file path");
    }

    #[test]
    fn test_validate_non_rspec_extensions() {
        let test_cases = vec![
            "spec/user_test.rb",
            "spec/user.rb",
            "spec/user_spec.py",
            "spec/user_spec.js",
        ];

        for case in test_cases {
            let result = ParsedFilePath::from_args(case, vec![]);
            assert!(result.is_err(), "Should reject {}", case);
            assert_eq!(
                result.unwrap_err(),
                "File must be an RSpec test file (*_spec.rb)"
            );
        }
    }
}
