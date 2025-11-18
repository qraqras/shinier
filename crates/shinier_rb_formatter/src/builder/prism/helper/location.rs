use ruby_prism::Location;

/// Checks if the location represents an "end" keyword.
pub fn is_end_keyword(location: &Option<Location>) -> bool {
    location
        .as_ref()
        .and_then(|l| std::str::from_utf8(l.as_slice()).ok())
        .map(|s| s == "end")
        .unwrap_or(false)
}
