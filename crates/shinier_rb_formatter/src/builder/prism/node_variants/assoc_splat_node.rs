use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

impl<'sh> Build for AssocSplatNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let value = self.value();
        group(array(&[string(SPLAT), value.build(context)]))
    }
}
