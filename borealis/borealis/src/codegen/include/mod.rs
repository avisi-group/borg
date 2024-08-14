use {proc_macro2::TokenStream, quote::ToTokens, std::fs};

#[allow(dead_code)]
mod bits;
#[allow(dead_code)]
mod dbt;
#[allow(dead_code)]
mod util;

pub fn get(filename: &str) -> TokenStream {
    let mut path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/codegen/include/").to_owned();
    path.push_str(filename);

    syn::parse_file(&fs::read_to_string(path).unwrap())
        .unwrap()
        .into_token_stream()
}

// tests not in the module itself because we don't want them emitted
#[cfg(test)]
mod tests {
    use crate::codegen::include::{
        bits::Bits,
        dbt::{BinaryOperation, ConstantValue, DynamicTranslator, NodeInner, Type, TypeKind},
    };

    #[test]
    fn sign_extend() {
        let bits = Bits::new(0xe57ba1c, 0x1c);
        let sign_extend = bits.sign_extend(64);
        assert_eq!(sign_extend.length(), 64);
        assert_eq!(sign_extend.value(), 0xfffffffffe57ba1c);
    }

    #[test]
    fn arithmetic_shift_right() {
        let bits = Bits::new(0xffff_ffd8 << 32, 0x40);
        let shift = bits.arithmetic_shift_right(32);
        assert_eq!(shift.length(), 64);
        assert_eq!(shift.value(), 0xffff_ffff_ffff_ffd8);
    }

    #[test]
    fn dbt_ergonomics() {
        let dbt = DynamicTranslator;

        let s0 = dbt.build(NodeInner::Constant {
            value: ConstantValue::Unsigned(0x1234),
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        });

        let s1 = dbt.build(NodeInner::ReadRegister {
            offset: s0,
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        });

        let s2 = dbt.build(NodeInner::Constant {
            value: ConstantValue::Unsigned(1),
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        });

        let s3 = dbt.build(NodeInner::BinaryOperation(BinaryOperation::Add(s1, s2)));

        let s4 = dbt.build(NodeInner::Constant {
            value: ConstantValue::Unsigned(0x4000),
            typ: Type {
                kind: TypeKind::Unsigned,
                width: 32,
            },
        });

        let _s5 = dbt.build(NodeInner::WriteRegister {
            value: s3,
            offset: s4,
        });
    }
}
