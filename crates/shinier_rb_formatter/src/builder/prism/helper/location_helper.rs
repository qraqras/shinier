use ruby_prism::Location;

pub fn read<'sh>(location: &'sh Location) -> &'sh str {
    std::str::from_utf8(location.as_slice()).unwrap_or_else(|_| panic!("Invalid UTF-8 in location: {:?}", location))
}

pub fn equals(location: &Location, keyword: &str) -> bool {
    read(location) == keyword
}

pub fn starts_with(location: &Location, prefix: &str) -> bool {
    read(location).starts_with(prefix)
}
