//! Rust code generation
//!
//! Watch out for `return quote!(...)` in these functions when they build up
//! quotes

use {
    crate::{
        codegen::{codegen_ident, codegen_type},
        rudder::{
            constant_value::ConstantValue,
            statement::{
                BinaryOperationKind, CastOperationKind, ShiftOperationKind, Statement,
                StatementKind, UnaryOperationKind,
            },
            Block, Function, PrimitiveType, PrimitiveTypeClass, Symbol, Type,
        },
    },
    proc_macro2::{Literal, TokenStream},
    quote::{format_ident, quote, ToTokens},
    std::sync::Arc,
    syn::Ident,
};

pub fn codegen_function(function: &Function) -> TokenStream {
    let name_ident = codegen_ident(function.name());
    let (return_type, parameters) = function.signature();

    let function_parameters = codegen_parameters(&parameters);
    let return_type = codegen_type(return_type);

    let fn_state = codegen_fn_state(&function, parameters.clone());

    let entry_block = get_block_fn_ident(&function.entry_block());

    let block_fns = function
        .entry_block()
        .iter()
        .map(|block| {
            let block_name = get_block_fn_ident(&block);
            let block_impl = codegen_block(block);

            quote! {
                // #[inline(always)] // enabling blows up memory usage during compilation (>1TB for 256 threads)
                fn #block_name(ctx: &mut X86TranslationContext, mut fn_state: FunctionState) -> #return_type {
                    let emitter = ctx.emitter();
                    #block_impl
                }
            }
        })
        .collect::<TokenStream>();

    let parameter_writes = parameters
        .iter()
        .map(|symbol| {
            let name = codegen_ident(symbol.name());
            quote!(emitter.write_variable(fn_state.#name.clone(), #name);)
        })
        .collect::<TokenStream>();

    let contents = quote! {
        #[inline(never)] // disabling increases compile time, perf impact not measured
        pub fn #name_ident(#function_parameters) -> #return_type {
            #fn_state

            {
                let emitter = ctx.emitter();
                #parameter_writes
            }

            return #entry_block(ctx, fn_state);

            #block_fns
        }
    };
    contents
}

pub fn codegen_parameters(parameters: &[Symbol]) -> TokenStream {
    let parameters =
        [quote!(ctx: &mut X86TranslationContext)]
            .into_iter()
            .chain(parameters.iter().map(|sym| {
                let name = codegen_ident(sym.name());
                quote!(#name: X86NodeRef)
            }));

    quote! {
        #(#parameters),*
    }
}

pub fn codegen_fn_state(function: &Function, parameters: Vec<Symbol>) -> TokenStream {
    let fn_state = {
        let fields = function
            .local_variables()
            .iter()
            .chain(&parameters)
            .map(|symbol| {
                let name = codegen_ident(symbol.name());

                quote! {
                    #name: X86SymbolRef,
                }
            })
            .collect::<TokenStream>();

        let field_inits = function
            .local_variables()
            .iter()
            .chain(&parameters)
            .map(|symbol| {
                let name = codegen_ident(symbol.name());

                quote! {
                    #name: ctx.create_symbol(),
                }
            })
            .collect::<TokenStream>();

        quote! {
            struct FunctionState {
                #fields
            }

            let fn_state = FunctionState {
                #field_inits
            };
        }
    };
    fn_state
}

pub fn codegen_block(block: Block) -> TokenStream {
    block
        .statements()
        .iter()
        .cloned()
        .map(codegen_stmt)
        .collect()
}

pub fn get_ident(stmt: &Statement) -> TokenStream {
    format_ident!("{}", stmt.name().to_string()).to_token_stream()
}

pub fn get_block_fn_ident(b: &Block) -> Ident {
    format_ident!("block_{}", b.name().as_ref())
}

/// Converts a rudder type to a `Type` value
fn codegen_type_instance(rudder: Arc<Type>) -> TokenStream {
    match &(*rudder) {
        Type::Primitive(primitive) => {
            let width = Literal::usize_unsuffixed(primitive.width());
            match primitive.tc {
                PrimitiveTypeClass::UnsignedInteger => quote! { Type {
                    kind: TypeKind::Unsigned,
                    width: #width,
                } },
                PrimitiveTypeClass::Void => todo!(),
                PrimitiveTypeClass::Unit => todo!(),
                PrimitiveTypeClass::SignedInteger => todo!(),
                PrimitiveTypeClass::FloatingPoint => todo!(),
            }
        }
        _ => todo!(),
    }
}

//
pub fn codegen_stmt(stmt: Statement) -> TokenStream {
    let stmt_name = format_ident!("{}", stmt.name().to_string());

    let statement_tokens = match stmt.kind() {
        StatementKind::Constant { value, typ } => match value {
            ConstantValue::UnsignedInteger(v) => {
                let v = Literal::usize_unsuffixed(v);
                let typ = codegen_type_instance(typ);
                quote!(emitter.constant(#v, #typ))
            }
            ConstantValue::SignedInteger(v) => {
                let v = Literal::isize_unsuffixed(v);
                quote!(#v)
            }
            ConstantValue::FloatingPoint(v) => {
                if v.is_infinite() {
                    quote!(1.1 / 0.0)
                } else {
                    let v = Literal::f64_unsuffixed(v);
                    quote!(#v)
                }
            }

            ConstantValue::Unit => quote!(()),
            ConstantValue::String(str) => {
                let string = str.to_string();
                quote!(#string)
            }
            ConstantValue::Rational(r) => {
                let numer = *r.numer();
                let denom = *r.denom();
                quote!(num_rational::Ratio::<i128>::new(#numer, #denom))
            }
        },
        StatementKind::ReadVariable { symbol } => {
            let symbol_ident = codegen_ident(symbol.name());
            quote! { emitter.read_variable(fn_state.#symbol_ident) }
        }
        StatementKind::WriteVariable { symbol, value } => {
            let var = codegen_ident(symbol.name());
            let value = get_ident(&value);
            quote! { emitter.write_variable(fn_state.#var, #value); }
        }
        StatementKind::ReadRegister { typ, offset } => {
            let offset = get_ident(&offset);
            let typ = codegen_type_instance(typ);
            quote! {
                emitter.read_register(#offset, #typ);
            }
        }
        StatementKind::WriteRegister { offset, value } => {
            let offset = get_ident(&offset);
            let typ = codegen_type(value.typ());
            let value = get_ident(&value);
            quote! {
                {
                    state.write_register::<#typ>(#offset as usize, #value);
                    tracer.write_register(#offset as usize, &#value);
                }
            }
        }
        // read `size` bytes at `offset`, return a Bits
        StatementKind::ReadMemory { offset, size } => {
            let offset = get_ident(&offset);
            let size = get_ident(&size);

            quote! {
                {
                    let mut buf = alloc::vec![0; #size as usize / 8];
                    state.read_memory(#offset, &mut buf);

                    let mut bytes = [0u8; 16];
                    bytes[..buf.len()].copy_from_slice(&buf);

                    Bits::new(u128::from_ne_bytes(bytes), #size as u16)
                }
            }
        }
        StatementKind::WriteMemory { offset, value } => {
            // OPTIMIZED VERSION:
            let offset = get_ident(&offset);

            // find size of value, either bundle.length or in type

            // emit match on this length to create mut pointer

            match &*value.typ() {
                Type::Primitive(PrimitiveType { .. }) => {
                    let value = get_ident(&value);
                    quote! {
                        state.write_memory(#offset, &#value.to_ne_bytes())
                    }
                }
                Type::Bits => {
                    let value = get_ident(&value);
                    quote! {
                        state.write_memory(#offset, &#value.value().to_ne_bytes()[..#value.length() as usize / 8])
                    }
                }
                _ => todo!(),
            }
        }
        StatementKind::ReadPc => quote!(todo!("read-pc")),
        StatementKind::WritePc { .. } => quote!(todo!("write-pc")),
        StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let left = get_ident(&lhs);
            let right = get_ident(&rhs);

            // // hard to decide whether this belongs, but since it's a Rust issue that u1
            // is // not like other types, casting is a codegen thing
            // match (lhs.typ().width_bits(), rhs.typ().width_bits()) {
            //     // both bools, do nothing
            //     (1, 1) => (),
            //     (1, _) => {
            //         let typ = codegen_type(rhs.typ());
            //         left = quote!(((#left) as #typ));
            //     }
            //     (_, 1) => {
            //         let typ = codegen_type(lhs.typ());
            //         right = quote!(((#right) as #typ));
            //     }
            //     // both not bools, do nothing
            //     (_, _) => (),
            // }

            let op = match kind {
                BinaryOperationKind::CompareEqual => quote! { (#left) == (#right) },
                BinaryOperationKind::Add => {
                    quote! { #left + #right }
                }
                BinaryOperationKind::Sub => quote! { (#left) - (#right) },
                BinaryOperationKind::Multiply => quote! { (#left) * (#right) },
                BinaryOperationKind::Divide => quote! { (#left) / (#right) },
                BinaryOperationKind::Modulo => quote! { (#left) % (#right) },
                BinaryOperationKind::And => quote! { (#left) & (#right) },
                BinaryOperationKind::Or => quote! { (#left) | (#right) },
                BinaryOperationKind::Xor => quote! { (#left) ^ (#right) },
                BinaryOperationKind::CompareNotEqual => quote! { (#left) != (#right) },
                BinaryOperationKind::CompareLessThan => quote! { (#left) < (#right) },
                BinaryOperationKind::CompareLessThanOrEqual => quote! { (#left) <= (#right) },
                BinaryOperationKind::CompareGreaterThan => quote! { (#left) > (#right) },
                BinaryOperationKind::CompareGreaterThanOrEqual => quote! { (#left) >= (#right) },
                BinaryOperationKind::PowI => quote! { (#left).powi(#right) },
            };

            quote! { (#op) }
        }
        StatementKind::UnaryOperation { kind, value } => {
            let value = get_ident(&value);

            match kind {
                UnaryOperationKind::Not => quote! {!#value},
                UnaryOperationKind::Negate => quote! {-#value},
                UnaryOperationKind::Complement => quote! {!#value},
                UnaryOperationKind::Power2 => quote! { (#value).pow(2) },
                UnaryOperationKind::Absolute => quote! { (#value).abs() },
                UnaryOperationKind::Ceil => quote! { (#value).ceil() },
                UnaryOperationKind::Floor => quote! { (#value).floor() },
                UnaryOperationKind::SquareRoot => quote! { (#value).sqrt() },
            }
        }
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => {
            let ident = get_ident(&value);
            let amount = get_ident(&amount);

            match kind {
                ShiftOperationKind::LogicalShiftLeft => quote! {#ident << #amount},
                ShiftOperationKind::LogicalShiftRight => quote! {#ident >> #amount},
                ShiftOperationKind::ArithmeticShiftRight => match &*value.typ() {
                    Type::Bits => {
                        quote! {
                            #ident.arithmetic_shift_right(#amount)
                        }
                    }
                    typ => unimplemented!("{typ:?}"),
                },
                ShiftOperationKind::RotateRight => todo!(),
                ShiftOperationKind::RotateLeft => todo!(),
            }
        }
        StatementKind::Call { target, args, tail } => {
            let ident = codegen_ident(target.name());
            let args = args.iter().map(get_ident);

            if tail {
                quote! {
                    return #ident(ctx, #(#args),*)
                }
            } else {
                quote! {
                    #ident(ctx, #(#args),*)
                }
            }
        }
        StatementKind::Cast { typ, value, kind } => codegen_cast(typ, value, kind),
        StatementKind::Jump { target } => {
            let target = get_block_fn_ident(&target);
            quote! {
               return #target(ctx, emitter, fn_state);
            }
        }
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => {
            let condition = get_ident(&condition);
            let true_target = get_block_fn_ident(&true_target);
            let false_target = get_block_fn_ident(&false_target);

            quote! {
                if #condition { return #true_target(ctx,  fn_state); } else { return #false_target(ctx,  fn_state); }
            }
        }
        StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        StatementKind::Return { value } => match value {
            Some(value) => {
                let name = codegen_ident(value.name());
                quote! { return #name; }
            }
            None => {
                quote! { return; }
            }
        },
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => {
            let condition = get_ident(&condition);
            let true_value = get_ident(&true_value);
            let false_value = get_ident(&false_value);
            quote! { if #condition { #true_value } else { #false_value } }
        }
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => {
            if let Type::Bits = &*value.typ() {
                let length = if let Type::Bits = &*length.typ() {
                    let length = get_ident(&length);
                    quote!(#length.value())
                } else {
                    let length = get_ident(&length);
                    quote!(u16::try_from(#length).unwrap())
                };

                let value = get_ident(&value);
                let start = get_ident(&start);

                quote! {
                    (Bits::new(((#value) >> (#start)).value(), #length))
                }
            } else {
                let typ = codegen_type(value.typ());

                let value = get_ident(&value);
                let start = get_ident(&start);
                let length = get_ident(&length);

                // todo: pre-cast length to u32

                quote! (
                    (
                        (#value >> #start) &
                        ((1 as #typ).checked_shl(#length as u32).map(|x| x - 1).unwrap_or(!0))
                    )
                )
            }
        }
        StatementKind::BitInsert {
            target: original_value,
            source: insert_value,
            start,
            length,
        } => {
            if let Type::Bits = &*original_value.typ() {
                let length = if let Type::Bits = &*length.typ() {
                    let length = get_ident(&length);
                    quote!(#length.value() as i128)
                } else {
                    let length = get_ident(&length);
                    quote!(#length as i128)
                };

                let original_value = get_ident(&original_value);
                let insert_value = get_ident(&insert_value);
                let start = get_ident(&start);

                quote! {
                    #original_value.insert(#insert_value.truncate(#length), #start)
                }
            } else {
                let typ = codegen_type(original_value.typ());

                let original_value = get_ident(&original_value);

                let insert_value = if let Type::Bits = &*insert_value.typ() {
                    let insert_value = get_ident(&insert_value);
                    quote!((#insert_value.value() as i128))
                } else {
                    let insert_value = get_ident(&insert_value);
                    quote!((#insert_value as i128))
                };

                let start = get_ident(&start);
                let length = get_ident(&length);

                // todo: pre-cast length to u32

                quote! {
                    {
                        let mask = !(((1 as #typ).checked_shl(#length as u32).map(|x| x - 1).unwrap_or(!0)) << #start);
                        (#original_value & mask) | (#insert_value << #start)
                    }
                }
            }
        }
        StatementKind::Panic(statements) => {
            let args = statements.iter().map(get_ident);

            quote!(panic!("{:?}", (#(#args),*)))
        }
        StatementKind::ReadElement { vector, index } => {
            let index_typ = index.typ();

            let vector = get_ident(&vector);
            let index = get_ident(&index);

            if let Type::Bits = &*index_typ {
                quote!(#vector[(#index.value()) as usize])
            } else {
                quote!(#vector[(#index) as usize])
            }
        }
        StatementKind::MutateElement {
            vector,
            value,
            index,
        } => {
            let vector = get_ident(&vector);
            let index = get_ident(&index);
            let value = get_ident(&value);
            // todo: support bundle indexes
            quote! {
                {
                    let mut local = #vector.clone();
                    local[(#index) as usize] = #value;
                    local
                }
            }
        }
        StatementKind::CreateStruct { typ, fields } => {
            let Type::Struct(field_types) = &*typ else {
                panic!()
            };

            let fields = fields
                .iter()
                .enumerate()
                .map(|(index, statement)| {
                    let (name, typ) = &field_types[index];
                    assert_eq!(*typ, statement.typ());
                    let field_name = codegen_ident(*name);
                    let value = get_ident(statement);
                    quote!(#field_name: #value,)
                })
                .collect::<TokenStream>();

            let typ = codegen_type(typ);
            quote!(#typ { #fields })
        }
        StatementKind::CreateEnum {
            typ,
            variant,
            value,
        } => {
            assert!(matches!(&*typ, Type::Enum(_)));
            let typ = codegen_type(typ);
            let variant = codegen_ident(variant);
            let value = get_ident(&value);
            quote!(#typ::#variant(#value))
        }
        StatementKind::CreateBits { value, length } => {
            let value = get_ident(&value);
            let length = get_ident(&length);
            quote!(Bits::new(#value, #length))
        }
        StatementKind::Assert { condition } => {
            let condition = get_ident(&condition);
            quote!(assert!(#condition))
        }
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => {
            let source_type = value.typ();
            let target_type = typ;
            let value_ident = get_ident(&value);
            let length_ident = get_ident(&length);

            match (&*source_type, &*target_type, kind) {
                (Type::Bits, Type::Bits, CastOperationKind::ZeroExtend) => {
                    quote!(#value_ident.zero_extend(#length_ident))
                }
                (Type::Bits, Type::Bits, CastOperationKind::SignExtend) => {
                    quote!(#value_ident.sign_extend(#length_ident))
                }
                (Type::Bits, Type::Bits, CastOperationKind::Truncate) => {
                    quote!(#value_ident.truncate(#length_ident))
                }
                _ => todo!(),
            }
        }
        StatementKind::SizeOf { value } => {
            let ident = get_ident(&value);
            match &*value.typ() {
                Type::Bits => quote!(#ident.length()),
                Type::ArbitraryLengthInteger => {
                    panic!("cannot get size of arbitrary length integer")
                }
                _ => {
                    // we represent all bitvector lengths as `u16`s
                    let length = u16::try_from(value.typ().width_bits()).unwrap();
                    quote!(#length)
                }
            }
        }
        StatementKind::MatchesEnum { value, variant } => {
            // matches!(value, Enum::Variant(_))

            let sum_type = value.typ();

            let Type::Enum(_) = &*sum_type else {
                unreachable!();
            };

            let ident = get_ident(&value);
            let sum_type = codegen_type(sum_type);
            let variant = codegen_ident(variant);

            quote! {
                matches!(#ident, #sum_type::#variant(_))
            }
        }
        StatementKind::UnwrapEnum { value, variant } => {
            let sum_type = value.typ();

            let Type::Enum(_) = &*sum_type else {
                unreachable!();
            };

            let ident = get_ident(&value);
            let sum_type = codegen_type(sum_type);
            let variant = codegen_ident(variant);

            quote! {
                match #ident {
                    #sum_type::#variant(inner) => inner,
                    _ => panic!("unwrap sum failed"),
                }
            }
        }
        StatementKind::ExtractField { value, field } => {
            let ident = get_ident(&value);
            let field = codegen_ident(field);
            quote!(#ident.#field)
        }
        StatementKind::UpdateField {
            original_value,
            field,
            field_value,
        } => {
            let ident = get_ident(&original_value);
            let value = get_ident(&field_value);
            let field = codegen_ident(field);
            quote! {
                {
                    let mut local = #ident;
                    local.#field = #value;
                    local
                }
            }
        }
        StatementKind::Undefined => quote!(Default::default()),
    };

    let msg = format!(" {} {stmt}", stmt.class());
    if stmt.has_value() {
        quote! {
            #[doc = #msg]
            let #stmt_name = #statement_tokens;
        }
    } else {
        quote! {
            #[doc = #msg]
            #statement_tokens;
        }
    }
}

pub fn codegen_cast(typ: Arc<Type>, value: Statement, kind: CastOperationKind) -> TokenStream {
    let source_type = value.typ();
    let target_type = typ;
    let ident = get_ident(&value);

    if source_type == target_type {
        log::warn!(
            "attemping to cast {:?} into same type ({})",
            value.name(),
            target_type
        );
        return quote! {
            ((#ident))
        };
    }

    match (&*source_type, &*target_type, kind) {
        // need to special case casting to booleans
        (
            Type::Primitive(_),
            Type::Primitive(PrimitiveType {
                element_width_in_bits: 1,
                tc: PrimitiveTypeClass::UnsignedInteger,
            }),
            _,
        ) => quote! {
            ((#ident) != 0)
        },

        // extract value before testing
        (
            Type::Bits,
            Type::Primitive(PrimitiveType {
                element_width_in_bits: 1,
                tc: PrimitiveTypeClass::UnsignedInteger,
            }),
            _,
        ) => quote! {
            ((#ident.value()) != 0)
        },

        // safe even if underlying rudder types are smaller than codegen'd rust
        // types (u7 -> u13 == u8 -> u16)
        (Type::Primitive(_), Type::Primitive(_), CastOperationKind::ZeroExtend) => {
            let target = codegen_type(target_type);
            quote! {
                (#ident as #target)
            }
        }

        (Type::Primitive(pt), Type::ArbitraryLengthInteger, CastOperationKind::ZeroExtend) => {
            match pt.tc {
                PrimitiveTypeClass::Void | PrimitiveTypeClass::Unit => {
                    panic!("cannot cast from void or unit")
                }
                PrimitiveTypeClass::UnsignedInteger | PrimitiveTypeClass::SignedInteger => {
                    let target = codegen_type(target_type);
                    quote! {
                        (#target::try_from(#ident).unwrap())
                    }
                }
                PrimitiveTypeClass::FloatingPoint => {
                    let target = codegen_type(target_type);
                    quote! {
                        (#ident as #target)
                    }
                }
            }
        }

        (
            Type::Primitive(PrimitiveType { tc, .. }),
            Type::Primitive(_),
            CastOperationKind::Truncate,
        ) => {
            assert!(target_type.width_bits() < source_type.width_bits());

            // create mask of length target
            let mask = Literal::u64_unsuffixed(
                1u64.checked_shl(u32::try_from(target_type.width_bits()).unwrap())
                    .map(|x| x - 1)
                    .unwrap_or(!0),
            );

            // cast to target type and apply mask to source
            let target = codegen_type(target_type);

            if let PrimitiveTypeClass::FloatingPoint = tc {
                // no mask needed for floating point casts
                quote!((#ident as #target))
            } else {
                // mask needed in case of truncating in between rust type widths
                quote!(((#ident as #target) & #mask))
            }
        }

        (Type::Bits, Type::Bits, CastOperationKind::Truncate) => {
            panic!("cannot truncate bits, target length not known by type system")
        }

        (Type::Bits, Type::ArbitraryLengthInteger, CastOperationKind::SignExtend) => {
            // void sail_signed(sail_int *rop, const lbits op)
            // {
            //   if (op.len == 0) {
            //     mpz_set_ui(*rop, 0);
            //   } else {
            //     mp_bitcnt_t sign_bit = op.len - 1;
            //     mpz_set(*rop, *op.bits);
            //     if (mpz_tstbit(*op.bits, sign_bit) != 0) {
            //       /* If sign bit is unset then we are done,
            //          otherwise clear sign_bit and subtract 2**sign_bit */
            //       mpz_set_ui(sail_lib_tmp1, 1);
            //       mpz_mul_2exp(sail_lib_tmp1, sail_lib_tmp1, sign_bit); /* 2**sign_bit */
            //       mpz_combit(*rop, sign_bit); /* clear sign_bit */
            //       mpz_sub(*rop, *rop, sail_lib_tmp1);
            //     }
            //   }
            // }
            quote! {
                {
                    let sign_bit = #ident.length() - 1;
                    let mut result = #ident.value() as i128;

                    if ((result >> sign_bit) & 1) == 1 {
                        // If sign bit is unset then we are done, otherwise clear sign_bit and subtract 2**sign_bit
                        let cleared_bit = result & !(1 << sign_bit);
                        result = cleared_bit - (1 << sign_bit)
                    }

                    result
                }
            }
        }

        (Type::Bits, Type::Primitive(_), CastOperationKind::Reinterpret) => {
            let target = codegen_type(target_type);
            quote! {
                (#ident.value() as #target)
            }
        }

        // todo: this should be a truncate??
        (Type::ArbitraryLengthInteger, Type::Primitive(_), CastOperationKind::Reinterpret) => {
            let target = codegen_type(target_type);
            quote! {
                (#ident as #target)
            }
        }

        (Type::Primitive(_), Type::Primitive(_), CastOperationKind::Reinterpret) => {
            let target = codegen_type(target_type);
            quote! {
                (#ident as #target)
            }
        }

        (Type::ArbitraryLengthInteger, Type::Bits, CastOperationKind::Convert) => {
            quote!(Bits::new(#ident as u128, 128))
        }

        (
            Type::Bits,
            Type::ArbitraryLengthInteger,
            CastOperationKind::Convert | CastOperationKind::ZeroExtend,
        ) => {
            let target_type = codegen_type(target_type);
            quote!((#ident.value() as #target_type))
        }

        // this type of cast replaces a lot of "create-bits"
        // todo ask tom about convert vs zeroextend
        (
            Type::Primitive(PrimitiveType {
                element_width_in_bits,
                ..
            }),
            Type::Bits,
            CastOperationKind::Convert | CastOperationKind::ZeroExtend,
        ) => {
            let width = u16::try_from(*element_width_in_bits).unwrap();
            // todo: maybe this as shouldn't be necessary?
            quote!(Bits::new(#ident as u128, #width))
        }

        (
            Type::Vector {
                element_type: source_type,
                ..
            },
            Type::Vector {
                element_count: 0,
                element_type: target_type,
            },
            CastOperationKind::Convert,
        ) => {
            assert_eq!(&**source_type, &**target_type);

            quote!(alloc::vec::Vec::from(#ident))
        }

        (
            Type::Vector {
                element_count: 0,
                element_type: source_type,
            },
            Type::Vector {
                element_count,
                element_type: target_type,
            },
            CastOperationKind::Convert,
        ) => {
            assert_eq!(&**source_type, &**target_type);

            //let element_type = codegen_type(target_type.clone());
            quote! {
                {
                    let mut buf = [Default::default(); #element_count];
                    buf.copy_from_slice(&#ident);
                    buf
                }
            }
        }

        (Type::Rational, Type::ArbitraryLengthInteger, CastOperationKind::Convert) => {
            quote! {
                #ident.to_integer()
            }
        }

        (Type::ArbitraryLengthInteger, Type::Rational, CastOperationKind::Convert) => {
            quote! {
                num_rational::Ratio::<i128>::from_integer(#ident)
            }
        }

        // Any can be cast to anything, but already has its type elided so we can just emit the
        // identifier and let rust sort out the type
        (Type::Any, _, _) => {
            quote! {
                (#ident)
            }
        }

        (src, tgt, knd) => panic!(
            "failed to generate code for cast of {:?} from {src} to {tgt} of kind {knd:?}",
            value.name()
        ),
    }
}
