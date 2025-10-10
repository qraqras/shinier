use ruby_prism::ConstantId;

pub fn constant_id_to_string(constant_id: &ConstantId) -> String {
    String::from_utf8(constant_id.as_slice().to_vec()).unwrap()
}
