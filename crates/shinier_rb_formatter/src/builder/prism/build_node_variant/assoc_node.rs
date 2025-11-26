use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_custom_location;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::AssocNode;
use ruby_prism::Node;

/// Builds AssocNode.
///
/// Formats hash associations in two styles:
/// - Hash Rocket Style: `key => value`
/// - Hash Label Style: `key: value`
///
/// Symbol keys using hash rocket syntax (`:key => value`) are automatically
/// converted to hash label syntax (`key: value`). Other keys retain the
/// hash rocket syntax.
///
/// Formats on a single line regardless of length, unless comments are present.
pub fn build_assoc_node(node: &AssocNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let key = node.key();
    let value = node.value();
    let operator_loc = node.operator_loc();

    match operator_loc {
        // Hash Rocket Style
        Some(o) => {
            // Insert line break only if there are comments between operator and value.
            let break_or_space = line_if_has_comments(o.end_offset(), value.location().start_offset(), ctx);

            // Determine the style based on the key type.
            match &key.as_symbol_node() {
                // SymbolNode: Convert to Hash Label Style
                Some(n) => group(array(&[
                    _build_node_as_hash_label_style(n.as_node(), ctx),
                    build_custom_location(o, ctx, ":"),
                    indent(array(&[break_or_space, build_node(value, ctx)])),
                ])),
                // Non-SymbolNode Key: Keep Hash Rocket Style
                None => group(array(&[
                    build_node(key, ctx),
                    space(),
                    build_location(o, ctx),
                    indent(array(&[break_or_space, build_node(value, ctx)])),
                ])),
            }
        }
        // Hash Label Style
        None => {
            // Insert line break only if there are comments between key and value.
            let break_or_space =
                line_if_has_comments(key.location().end_offset(), value.location().start_offset(), ctx);

            group(array(&[
                build_node(key, ctx),
                indent(array(&[break_or_space, build_node(value, ctx)])),
            ]))
        }
    }
}

fn _build_node_as_hash_label_style(node: Node<'_>, ctx: &mut BuildContext) -> Option<Document> {
    ctx.hash_label_style = true;
    let built = build_node(node, ctx);
    ctx.hash_label_style = false;
    built
}
