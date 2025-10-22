use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use num_bigint::{BigInt, Sign};
use ruby_prism::Integer;

impl Buildable<'_> for Integer<'_> {
    fn build(&self) -> Document {
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
