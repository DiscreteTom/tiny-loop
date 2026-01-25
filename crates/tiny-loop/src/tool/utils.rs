/// Truncate text content with pagination support
pub fn truncate_text(content: String, start: usize, end: usize) -> String {
    let end_idx = end.min(content.len());
    let total_len = content.len();

    let mut result: String = content
        .chars()
        .skip(start)
        .take(end_idx.saturating_sub(start))
        .collect();

    if end_idx < total_len {
        result.push_str(&format!(
            "\n\n---\ntruncated [{}/{} chars]",
            end_idx, total_len
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_truncation() {
        let result = truncate_text("hello".to_string(), 0, 5000);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_truncation() {
        let content = "a".repeat(6000);
        let result = truncate_text(content, 0, 5000);
        assert_eq!(
            result,
            format!("{}\n\n---\ntruncated [5000/6000 chars]", "a".repeat(5000))
        );
    }

    #[test]
    fn test_custom_range() {
        let result = truncate_text("0123456789".to_string(), 2, 5);
        assert_eq!(result, "234\n\n---\ntruncated [5/10 chars]");
    }

    #[test]
    fn test_with_start() {
        let result = truncate_text("0123456789".to_string(), 5, 5000);
        assert_eq!(result, "56789");
    }

    #[test]
    fn test_end_beyond_length() {
        let result = truncate_text("hello".to_string(), 0, 100);
        assert_eq!(result, "hello");
    }
}
