use ruby_prism::Location;

/// Reads the content of the given Location as a UTF-8 string slice.
pub fn read<'sh>(location: &'sh Location) -> &'sh str {
    std::str::from_utf8(location.as_slice()).unwrap_or_else(|_| panic!("Invalid UTF-8 in location: {:?}", location))
}

/// Compares the content of the given Location with the provided keyword.
pub fn equals(location: &Location, keyword: &str) -> bool {
    read(location) == keyword
}

/// Checks if the content of the given Location starts with the provided prefix.
pub fn starts_with(location: &Location, prefix: &str) -> bool {
    read(location).starts_with(prefix)
}

/// Finds the start and end offsets of the given keyword.
pub fn find(location: &Location, keyword: &str) -> Option<(usize, usize)> {
    read(location).find(keyword).map(|index| {
        let start = location.start_offset() + index;
        (start, start + keyword.len())
    })
}
