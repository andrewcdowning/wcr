#[cfg(test)]
mod tests {
    use wcr::FileInfo;
    use wcr::count;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world.  I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected)
    }
}