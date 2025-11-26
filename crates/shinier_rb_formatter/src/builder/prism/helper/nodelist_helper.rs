use ruby_prism::NodeList;

/// Returns the start offset of the first node in the list.
pub fn nodelist_start_offset(nodelist: &NodeList) -> Option<usize> {
    nodelist.iter().next().map(|n| n.location().start_offset())
}

/// Returns the end offset of the last node in the list.
pub fn nodelist_end_offset(nodelist: &NodeList) -> Option<usize> {
    nodelist.iter().last().map(|n| n.location().end_offset())
}
