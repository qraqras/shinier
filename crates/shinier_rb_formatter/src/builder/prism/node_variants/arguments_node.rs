use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::line;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

impl<'sh> Build for ArgumentsNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let arguments = self.arguments();
        group(arguments.build(context, &array(&[string(COMMA), line()])))
    }
}
