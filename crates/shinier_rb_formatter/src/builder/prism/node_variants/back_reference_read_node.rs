use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::BackReferenceReadNode;

impl<'sh> Build for BackReferenceReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let name = self.name();
        name.build(context)
    }
}
