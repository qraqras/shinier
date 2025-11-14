use crate::Document;

pub struct LayoutParamSymbolNode {
    pub location: Document,
}

pub fn layout_symbol_node(param: LayoutParamSymbolNode) -> Document {
    param.location
}
