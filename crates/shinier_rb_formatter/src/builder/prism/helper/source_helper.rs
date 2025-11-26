pub fn find_offset(source: &[u8], start_offset: usize, end_offset: usize, keyword: &str) -> Option<(usize, usize)> {
    let keyword_bytes = keyword.as_bytes();
    let mut offset = start_offset;
    while offset + keyword_bytes.len() <= end_offset {
        if &source[offset..offset + keyword_bytes.len()] == keyword_bytes {
            return Some((offset, offset + keyword_bytes.len()));
        }
        offset += 1;
    }
    None
}
