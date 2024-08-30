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
        FN_ALLOWLIST,
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

    let fn_state = codegen_fn_state(&function, parameters.clone());

    let block_fns = function
        .entry_block()
        .iter()
        .map(|block| {
            let block_name = get_block_fn_ident(&block);
            let block_impl = codegen_block(block);

            quote! {
                // #[inline(always)] // enabling blows up memory usage during compilation (>1TB for 256 threads)
                fn #block_name(ctx: &mut X86TranslationContext, fn_state: &FunctionState) -> BlockResult  {
                    #block_impl
                }
            }
        })
        .collect::<TokenStream>();

    let num_blocks = function.entry_block().iter().count();
    let block_fn_names = function
        .entry_block()
        .iter()
        .map(|block| {
            let fn_name = get_block_fn_ident(&block);
            quote!(#fn_name, )
        })
        .collect::<TokenStream>();

    let parameter_writes = parameters
        .iter()
        .map(|symbol| {
            let name = codegen_ident(symbol.name());
            quote!(ctx.emitter().write_variable(fn_state.#name.clone(), #name);)
        })
        .collect::<TokenStream>();

    let body = if FN_ALLOWLIST.contains(&function.name().as_ref()) {
        quote! {
            #fn_state

            {
                let emitter = ctx.emitter();
                #parameter_writes
            }

            const BLOCK_FUNCTIONS: [fn(&mut X86TranslationContext, &FunctionState) -> BlockResult; #num_blocks] = [#block_fn_names];

            fn lookup_block_idx_by_ref(block_refs: &[X86BlockRef], block: X86BlockRef) -> usize {
                block_refs.iter().position(|r| *r == block).unwrap()
            }

            enum Block {
                Static(usize),
                Dynamic(usize),
            }

            let mut block_queue = alloc::vec![Block::Static(0)];

            while let Some(block) = block_queue.pop() {
                let result = match block {
                    Block::Static(i) => {
                        log::debug!("static block {i}");
                        BLOCK_FUNCTIONS[i](ctx, &fn_state)
                    }
                    Block::Dynamic(i) => {
                        log::debug!("dynamic block {i}");
                        ctx.emitter().set_current_block(fn_state.block_refs[i].clone());
                        BLOCK_FUNCTIONS[i](ctx, &fn_state)
                    }
                };

                match result {
                    BlockResult::None => {},
                    BlockResult::Static(block) => {
                        block_queue.push(Block::Static(lookup_block_idx_by_ref(&fn_state.block_refs, block)));
                    }
                    BlockResult::Dynamic(b0, b1) => {
                        block_queue.push(Block::Dynamic(lookup_block_idx_by_ref(&fn_state.block_refs, b0)));
                        block_queue.push(Block::Dynamic(lookup_block_idx_by_ref(&fn_state.block_refs, b1)));
                    },
                    BlockResult::Return(node) => {
                        ctx.emitter().set_current_block(fn_state.exit_block_ref.clone());
                        return node;
                    }
                }
            }

            unreachable!();

            #block_fns
        }
    } else {
        quote!(todo!())
    };

    quote! {
        #[inline(never)] // disabling increases compile time, perf impact not measured
        pub fn #name_ident(#function_parameters) -> X86NodeRef {
            #body
        }
    }
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

    let block_ref_inits = function
        .entry_block()
        .iter()
        .map(|_| quote!(ctx.create_block(),))
        .collect::<TokenStream>();

    let num_blocks = function.entry_block().iter().count();

    quote! {
        struct FunctionState {
            #fields
            block_refs: [X86BlockRef; #num_blocks],
            exit_block_ref: X86BlockRef,
        }

        let fn_state = FunctionState {
            #field_inits
            block_refs: [#block_ref_inits],
            exit_block_ref: ctx.create_block(),
        };
    }
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
    format_ident!("block_{}", b.index())
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
                PrimitiveTypeClass::SignedInteger => quote! { Type {
                    kind: TypeKind::Signed,
                    width: #width,
                } },
                PrimitiveTypeClass::FloatingPoint => quote! { Type {
                    kind: TypeKind::Floating,
                    width: #width,
                } },
            }
        }
        Type::ArbitraryLengthInteger => {
            quote! {
                Type {
                    kind: TypeKind::Signed,
                    width: 128,
                }
            }
        }
        Type::Bits => {
            quote! {
                Type {
                    kind: TypeKind::Unsigned,
                    width: todo!(),
                }
            }
        }
        t => panic!("todo codegen type instance: {t:?}"),
    }
}

/// Converts a rudder type to a `Type` value
fn codegen_constant_type_instance(value: &ConstantValue, typ: Arc<Type>) -> TokenStream {
    match &(*typ) {
        Type::Primitive(primitive) => {
            let width = Literal::usize_unsuffixed(primitive.width());
            match primitive.tc {
                PrimitiveTypeClass::UnsignedInteger => quote! { Type {
                    kind: TypeKind::Unsigned,
                    width: #width,
                } },
                PrimitiveTypeClass::Void => todo!(),
                PrimitiveTypeClass::Unit => quote! { Type {
                    kind: TypeKind::Unsigned,
                    width: 0,
                } },
                PrimitiveTypeClass::SignedInteger => quote! { Type {
                    kind: TypeKind::Signed,
                    width: #width,
                } },
                PrimitiveTypeClass::FloatingPoint => quote! { Type {
                    kind: TypeKind::Floating,
                    width: #width,
                } },
            }
        }
        Type::ArbitraryLengthInteger => {
            let ConstantValue::SignedInteger(cv) = value else {
                panic!();
            };

            let width = u16::try_from(((*cv as usize) + 1).next_power_of_two().ilog2()).unwrap();

            quote! {
                Type {
                    kind: TypeKind::Signed,
                    width: #width,
                }
            }
        }
        Type::Bits => {
            let ConstantValue::UnsignedInteger(cv) = value else {
                panic!();
            };

            let width = u16::try_from((cv + 1).next_power_of_two().ilog2()).unwrap();

            quote! {
                Type {
                    kind: TypeKind::Unsigned,
                    width: #width,
                }
            }
        }
        Type::String => {
            quote! {
                Type {
                    kind: TypeKind::Unsigned,
                    width: 0,
                }
            }
        }
        Type::Union { width } => {
            let width = u16::try_from(*width).unwrap();
            quote! {
                Type {
                    kind: TypeKind::Unsigned,
                    width: #width,
                }
            }
        }
        t => panic!("todo codegen type instance: {t:?}"),
    }
}

//
pub fn codegen_stmt(stmt: Statement) -> TokenStream {
    let stmt_name = format_ident!("{}", stmt.name().to_string());

    let statement_tokens = match stmt.kind() {
        StatementKind::Constant { value, typ } => {
            let typ = codegen_constant_type_instance(&value, typ);
            match value {
                ConstantValue::UnsignedInteger(v) => {
                    let v = Literal::usize_unsuffixed(v);
                    quote!(ctx.emitter().constant(#v, #typ))
                }
                ConstantValue::SignedInteger(v) => {
                    quote!(ctx.emitter().constant(#v as u64, #typ))
                }
                ConstantValue::FloatingPoint(v) => {
                    quote!(ctx.emitter().constant(#v as u64, #typ))
                }
                ConstantValue::Unit => quote!(ctx.emitter().constant(0, #typ)),
                ConstantValue::Rational(_) | ConstantValue::String(_) => todo!(),
            }
        }
        StatementKind::ReadVariable { symbol } => {
            let symbol_ident = codegen_ident(symbol.name());
            quote! { ctx.emitter().read_variable(fn_state.#symbol_ident.clone()) }
        }
        StatementKind::WriteVariable { symbol, value } => {
            let symbol_ident = codegen_ident(symbol.name());
            let value = get_ident(&value);
            quote! { ctx.emitter().write_variable(fn_state.#symbol_ident.clone(), #value.clone()); }
        }
        StatementKind::ReadRegister { typ, offset } => {
            let offset = get_ident(&offset);
            let typ = codegen_type_instance(typ);
            quote! {
                ctx.emitter().read_register(#offset.clone(), #typ);
            }
        }
        StatementKind::WriteRegister { offset, value } => {
            let offset = get_ident(&offset);
            let value = get_ident(&value);
            quote! {
                ctx.emitter().write_register(#offset.clone(), #value.clone());
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

            let kind = match kind {
                BinaryOperationKind::Add => quote!(Add),
                BinaryOperationKind::Sub => quote!(Sub),
                BinaryOperationKind::Multiply => quote!(Multiply),
                BinaryOperationKind::Divide => quote!(Divide),
                BinaryOperationKind::Modulo => quote!(Modulo),
                BinaryOperationKind::And => quote!(And),
                BinaryOperationKind::Or => quote!(Or),
                BinaryOperationKind::Xor => quote!(Xor),
                BinaryOperationKind::CompareEqual => quote!(CompareEqual),
                BinaryOperationKind::CompareNotEqual => quote!(CompareNotEqual),
                BinaryOperationKind::CompareLessThan => quote!(CompareLessThan),
                BinaryOperationKind::CompareLessThanOrEqual => quote!(CompareLessThanOrEqual),
                BinaryOperationKind::CompareGreaterThan => quote!(CompareGreaterThan),
                BinaryOperationKind::CompareGreaterThanOrEqual => quote!(CompareGreaterThanOrEqual),
                BinaryOperationKind::PowI => quote!(PowI),
            };

            quote! { ctx.emitter().binary_operation(BinaryOperationKind::#kind(#left.clone(), #right.clone())) }
        }
        StatementKind::UnaryOperation { kind, value } => {
            let value = get_ident(&value);

            let kind = match kind {
                UnaryOperationKind::Not => quote!(Not),
                UnaryOperationKind::Negate => quote!(Negate),
                UnaryOperationKind::Complement => quote!(Complement),
                UnaryOperationKind::Power2 => quote!(Power2),
                UnaryOperationKind::Absolute => quote!(Absolute),
                UnaryOperationKind::Ceil => quote!(Ceil),
                UnaryOperationKind::Floor => quote!(Floor),
                UnaryOperationKind::SquareRoot => quote!(SquareRoot),
            };

            quote! { ctx.emitter().unary_operation(UnaryOperationKind::#kind(#value.clone())) }
        }
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => {
            let value = get_ident(&value);
            let amount = get_ident(&amount);

            let kind = match kind {
                ShiftOperationKind::LogicalShiftLeft => quote!(LogicalShiftLeft),
                ShiftOperationKind::LogicalShiftRight => quote!(LogicalShiftRight),
                ShiftOperationKind::ArithmeticShiftRight => quote!(ArithmeticShiftRight),
                ShiftOperationKind::RotateRight => quote!(RotateRight),
                ShiftOperationKind::RotateLeft => quote!(RotateLeft),
            };

            quote! { ctx.emitter().shift(#value.clone(), #amount.clone(), ShiftOperationKind::#kind) }
        }
        StatementKind::Call { target, args, .. } => {
            let ident = codegen_ident(target.name());
            let args = args.iter().map(get_ident);

            // if tail {
            //     quote! {
            //         return #ident(ctx, #(#args),*)
            //     }
            // } else {
            //
            // }
            // todo check me
            quote! {
                #ident(ctx, #(#args),*)
            }
        }
        StatementKind::Cast { typ, value, kind } => {
            let typ = codegen_type_instance(typ);
            let value = get_ident(&value);

            let kind = match kind {
                CastOperationKind::ZeroExtend => quote!(ZeroExtend),
                CastOperationKind::SignExtend => quote!(SignExtend),
                CastOperationKind::Truncate => quote!(Truncate),
                CastOperationKind::Reinterpret => quote!(Reinterpret),
                CastOperationKind::Convert => quote!(Convert),
                CastOperationKind::Broadcast => quote!(Broadcast),
            };

            quote! { ctx.emitter().cast(#value.clone(), #typ, CastOperationKind::#kind) }
        }
        StatementKind::Jump { target } => {
            let target_index = target.index();
            quote! {
                return ctx.emitter().jump(fn_state.block_refs[#target_index].clone())
            }
        }
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => {
            let condition = get_ident(&condition);
            let true_index = true_target.index();
            let false_index = false_target.index();

            quote! {
                return ctx.emitter().branch(#condition.clone(), fn_state.block_refs[#true_index].clone(), fn_state.block_refs[#false_index].clone())
            }
        }
        StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        StatementKind::Return { value } => match value {
            Some(value) => {
                let name = codegen_ident(value.name());
                quote! { return BlockResult::Return(#name); }
            }
            None => {
                quote! {
                    let v = ctx.emitter().constant(0, Type {
                        kind: TypeKind::Unsigned,
                        width: 0,
                    });
                    return BlockResult::Return(v);
                }
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
            quote! { ctx.emitter().select(#condition, #true_value, #false_value) }
        }
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => {
            let value = get_ident(&value);
            let start = get_ident(&start);
            let length = get_ident(&length);
            quote! { ctx.emitter().bit_extract(#value.clone(), #start.clone(), #length.clone()) }
        }
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => {
            let target = get_ident(&target);
            let source = get_ident(&source);
            let start = get_ident(&start);
            let length = get_ident(&length);
            quote! {ctx.emitter().bit_insert(#target.clone(), #source.clone(), #start.clone(), #length.clone())}
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
        StatementKind::MatchesUnion { value, variant } => {
            // // matches!(value, Enum::Variant(_))

            // let sum_type = value.typ();

            // let Type::Enum(_) = &*sum_type else {
            //     unreachable!();
            // };

            // let ident = get_ident(&value);
            // let sum_type = codegen_type(sum_type);
            // let variant = codegen_ident(variant);

            // quote! {
            //     matches!(#ident, #sum_type::#variant(_))
            // }
            todo!()
        }
        StatementKind::UnwrapUnion { value, variant } => {
            // let sum_type = value.typ();

            // let Type::Enum(_) = &*sum_type else {
            //     unreachable!();
            // };

            // let ident = get_ident(&value);
            // let sum_type = codegen_type(sum_type);
            // let variant = codegen_ident(variant);

            // quote! {
            //     match #ident {
            //         #sum_type::#variant(inner) => inner,
            //         _ => panic!("unwrap sum failed"),
            //     }
            // }
            todo!()
        }

        StatementKind::Undefined => quote!(Default::default()),
    };

    let msg = format!(" {stmt}");
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
