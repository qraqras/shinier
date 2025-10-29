use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use num_bigint::{BigInt, Sign};
use ruby_prism::Integer;

impl<'sh> Build for Integer<'sh> {
    fn __build__(&self, _context: &mut BuildContext) -> Document {
        let (negative, digits) = self.to_u32_digits();

        if digits.is_empty() {
            return string("0");
        }

        // u32配列をBigIntに変換
        let sign = if negative { Sign::Minus } else { Sign::Plus };
        let bigint = BigInt::from_slice(sign, digits);

        // BigIntの文字列表現を使用
        string(bigint.to_string())
    }
}
