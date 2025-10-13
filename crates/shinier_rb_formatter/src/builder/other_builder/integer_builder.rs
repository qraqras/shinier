use crate::builder::Buildable;
use crate::doc::{Doc, text};
use num_bigint::{BigInt, Sign};
use ruby_prism::Integer;

impl Buildable<'_> for Integer<'_> {
    fn build(&self) -> Doc {
        let (negative, digits) = self.to_u32_digits();

        if digits.is_empty() {
            return text("0");
        }

        // u32配列をBigIntに変換
        let sign = if negative { Sign::Minus } else { Sign::Plus };
        let bigint = BigInt::from_slice(sign, digits);

        // BigIntの文字列表現を使用
        text(bigint.to_string())
    }
}
