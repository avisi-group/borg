//! Rust code generation
//!
//! Watch out for `return quote!(...)` in these functions when they build up
//! quotes

use {
    crate::{
        codegen::{codegen_ident, codegen_type},
        fn_is_allowlisted,
        rudder::{
            constant_value::ConstantValue,
            statement::{
                BinaryOperationKind, CastOperationKind, Flag, ShiftOperationKind, Statement,
                StatementKind, UnaryOperationKind,
            },
            Block, Function, PrimitiveType, PrimitiveTypeClass, Symbol, Type,
        },
        util::{signed_smallest_width_of_value, unsigned_smallest_width_of_value},
    },
    proc_macro2::{Literal, TokenStream},
    quote::{format_ident, quote},
    std::sync::Arc,
    syn::Ident,
};

pub fn codegen_function(function: &Function) -> TokenStream {
    let name_ident = codegen_ident(function.name());
    let (_return_type, parameters) = function.signature();

    // if let Type::Tuple(ts) = &*return_type {
    //     let elements = repeat(quote!(X86NodeRef)).take(ts.len());
    //     quote!((#(#elements),*))

    // } else {
    //     quote!(X86NodeRef)
    // };
    let return_type = quote!(X86NodeRef);

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

    let body = if fn_is_allowlisted(function.name()) {
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
                    BlockResult::Static(block) => {
                        let idx = lookup_block_idx_by_ref(&fn_state.block_refs, block);
                        log::debug!("block result: static({idx})");
                        block_queue.push(Block::Static(idx));
                    }
                    BlockResult::Dynamic(b0, b1) => {
                        let i0 = lookup_block_idx_by_ref(&fn_state.block_refs, b0);
                        let i1 = lookup_block_idx_by_ref(&fn_state.block_refs, b1);
                        log::debug!("block result: dynamic({i0}, {i1})");
                        block_queue.push(Block::Dynamic(i0));
                        block_queue.push(Block::Dynamic(i1));
                    },
                    BlockResult::Return => {
                        log::debug!("block result: return");
                        ctx.emitter().jump(fn_state.exit_block_ref.clone());
                    }
                    BlockResult::Panic => {
                        log::debug!("block result: panic");
                        // unreachable but inserted just to make sure *every* block has a path to the exit block
                        ctx.emitter().jump(fn_state.exit_block_ref.clone());
                    }
                }
            }

            ctx.emitter().set_current_block(fn_state.exit_block_ref.clone());

            return ctx.emitter().read_variable(fn_state.borealis_fn_return_value.clone());

            #block_fns
        }
    } else {
        quote!(todo!())
    };

    quote! {
        #[inline(never)] // disabling increases compile time, perf impact not measured
        pub fn #name_ident(#function_parameters) -> #return_type {
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
            borealis_fn_return_value: X86SymbolRef,
            block_refs: [X86BlockRef; #num_blocks],
            exit_block_ref: X86BlockRef,
        }

        let fn_state = FunctionState {
            #field_inits
            borealis_fn_return_value: ctx.create_symbol(),
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
                    width: 64, // bad! bad!
                }
            }
        }
        Type::Bits => {
            quote! {
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,// todo: bad
                }
            }
        }
        Type::Vector {
            element_count,
            element_type,
        } => {
            let element_width = u16::try_from(element_type.width_bits()).unwrap();
            let element_type = codegen_type_instance(element_type.clone());
            let element_count = Literal::u16_suffixed(u16::try_from(*element_count).unwrap());

            quote! {
                Type {
                    kind: TypeKind::Vector { length: #element_count, element: alloc::boxed::Box::new(#element_type) },
                    width: #element_count * #element_width,
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

            let width = signed_smallest_width_of_value(*cv);

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

            let width = unsigned_smallest_width_of_value(*cv);

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
        StatementKind::Constant { value, typ } => codegen_constant_value(value, typ),
        StatementKind::ReadVariable { symbol } => {
            let symbol_ident = codegen_ident(symbol.name());
            quote! { ctx.emitter().read_variable(fn_state.#symbol_ident.clone()) }
        }
        StatementKind::WriteVariable { symbol, value } => {
            let symbol_ident = codegen_ident(symbol.name());

            quote! { ctx.emitter().write_variable(fn_state.#symbol_ident.clone(), #value.clone()); }
        }
        StatementKind::ReadRegister { typ, offset } => {
            let typ = codegen_type_instance(typ);
            quote! {
                ctx.emitter().read_register(#offset.clone(), #typ);
            }
        }
        StatementKind::WriteRegister { offset, value } => {
            quote! {
                ctx.emitter().write_register(#offset.clone(), #value.clone());
            }
        }
        // read `size` bytes at `offset`, return a Bits
        StatementKind::ReadMemory { offset, size } => {
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

            // find size of value, either bundle.length or in type

            // emit match on this length to create mut pointer

            match &*value.typ() {
                Type::Primitive(PrimitiveType { .. }) => {
                    quote! {
                        state.write_memory(#offset, &#value.to_ne_bytes())
                    }
                }
                Type::Bits => {
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
            let left = lhs; // todo:
            let right = rhs;

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
            let true_index = true_target.index();
            let false_index = false_target.index();

            quote! {
                return ctx.emitter().branch(#condition.clone(), fn_state.block_refs[#true_index].clone(), fn_state.block_refs[#false_index].clone())
            }
        }
        StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        StatementKind::Return { value } => {
            let name = codegen_ident(value.name());
            quote! {
                ctx.emitter().write_variable(fn_state.borealis_fn_return_value.clone(), #name);
                return BlockResult::Return;
            }
        }
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => {
            quote! { ctx.emitter().select(#condition, #true_value, #false_value) }
        }
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => {
            quote! { ctx.emitter().bit_extract(#value.clone(), #start.clone(), #length.clone()) }
        }
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => {
            quote! {ctx.emitter().bit_insert(#target.clone(), #source.clone(), #start.clone(), #length.clone())}
        }
        StatementKind::Panic(statement) => {
            quote! {
                ctx.emitter().panic(#statement);
                return BlockResult::Panic;
            }
        }
        StatementKind::ReadElement { vector, index } => {
            quote!(ctx.emitter().read_element(#vector, #index))
        }
        StatementKind::AssignElement {
            vector,
            value,
            index,
        } => {
            quote!(ctx.emitter().mutate_element(#vector, #index, #value))
        }

        StatementKind::CreateBits { value, length } => {
            quote!(Bits::new(#value, #length))
        }
        StatementKind::Assert { condition } => {
            quote!(ctx.emitter().assert(#condition))
        }
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => {
            let source_type = value.typ();
            let target_type = typ;

            match (&*source_type, &*target_type, kind) {
                (Type::Bits, Type::Bits, CastOperationKind::ZeroExtend) => {
                    quote!(#value.zero_extend(#length))
                }
                (Type::Bits, Type::Bits, CastOperationKind::SignExtend) => {
                    quote!(#value.sign_extend(#length))
                }
                (Type::Bits, Type::Bits, CastOperationKind::Truncate) => {
                    quote!(#value.truncate(#length))
                }
                _ => todo!(),
            }
        }
        StatementKind::SizeOf { value } => {
            match &*value.typ() {
                Type::Bits => quote!(#value.length()),
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
        StatementKind::MatchesUnion { .. } => {
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
        StatementKind::UnwrapUnion { .. } => {
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
        StatementKind::TupleAccess { index, source } => {
            let index = Literal::usize_unsuffixed(index);
            quote!(#source.#index)
        }
        StatementKind::GetFlag { flag, operation } => {
            let flag = match flag {
                Flag::N => quote!(N),
                Flag::Z => quote!(Z),
                Flag::C => quote!(C),
                Flag::V => quote!(V),
            };
            quote!(ctx.emitter().get_flag(Flag::#flag, #operation.clone()))
        }
        StatementKind::CreateTuple(values) => {
            quote!((#(#values),*))
        }
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

    if source_type == target_type {
        log::warn!(
            "attemping to cast {:?} into same type ({})",
            value.name(),
            target_type
        );
        return quote!(#value);
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
            ((#value) != 0)
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
            ((#value.value()) != 0)
        },

        // safe even if underlying rudder types are smaller than codegen'd rust
        // types (u7 -> u13 == u8 -> u16)
        (Type::Primitive(_), Type::Primitive(_), CastOperationKind::ZeroExtend) => {
            let target = codegen_type(target_type);
            quote! {
                (#value as #target)
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
                        (#target::try_from(#value).unwrap())
                    }
                }
                PrimitiveTypeClass::FloatingPoint => {
                    let target = codegen_type(target_type);
                    quote! {
                        (#value as #target)
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
                quote!((#value as #target))
            } else {
                // mask needed in case of truncating in between rust type widths
                quote!(((#value as #target) & #mask))
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
                    let sign_bit = #value.length() - 1;
                    let mut result = #value.value() as i128;

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
                (#value.value() as #target)
            }
        }

        // todo: this should be a truncate??
        (Type::ArbitraryLengthInteger, Type::Primitive(_), CastOperationKind::Reinterpret) => {
            let target = codegen_type(target_type);
            quote! {
                (#value as #target)
            }
        }

        (Type::Primitive(_), Type::Primitive(_), CastOperationKind::Reinterpret) => {
            let target = codegen_type(target_type);
            quote! {
                (#value as #target)
            }
        }

        (Type::ArbitraryLengthInteger, Type::Bits, CastOperationKind::Convert) => {
            quote!(Bits::new(#value as u128, 128))
        }

        (
            Type::Bits,
            Type::ArbitraryLengthInteger,
            CastOperationKind::Convert | CastOperationKind::ZeroExtend,
        ) => {
            let target_type = codegen_type(target_type);
            quote!((#value.value() as #target_type))
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
            quote!(Bits::new(#value as u128, #width))
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

            quote!(alloc::vec::Vec::from(#value))
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
                    buf.copy_from_slice(&#value);
                    buf
                }
            }
        }

        (Type::Rational, Type::ArbitraryLengthInteger, CastOperationKind::Convert) => {
            quote! {
                #value.to_integer()
            }
        }

        (Type::ArbitraryLengthInteger, Type::Rational, CastOperationKind::Convert) => {
            quote! {
                num_rational::Ratio::<i128>::from_integer(#value)
            }
        }

        // Any can be cast to anything, but already has its type elided so we can just emit the
        // identifier and let rust sort out the type
        (Type::Any, _, _) => {
            quote! {
                (#value)
            }
        }

        (src, tgt, knd) => panic!(
            "failed to generate code for cast of {:?} from {src} to {tgt} of kind {knd:?}",
            value.name()
        ),
    }
}

fn codegen_constant_value(value: ConstantValue, typ: Arc<Type>) -> TokenStream {
    let typ_instance = codegen_constant_type_instance(&value, typ.clone());
    match value {
        ConstantValue::UnsignedInteger(v) => {
            quote!(ctx.emitter().constant(#v, #typ_instance))
        }
        ConstantValue::SignedInteger(v) => {
            quote!(ctx.emitter().constant(#v as u64, #typ_instance))
        }
        ConstantValue::FloatingPoint(v) => {
            quote!(ctx.emitter().constant(#v as u64, #typ_instance))
        }
        ConstantValue::Unit => quote!(ctx.emitter().constant(0, #typ_instance)),
        ConstantValue::String(s) => {
            let str = s.as_ref();
            quote!(#str)
        }
        ConstantValue::Rational(_) => todo!(),

        ConstantValue::Tuple(values) => {
            let Type::Tuple(types) = &*typ else { panic!() };
            let values = values
                .iter()
                .cloned()
                .zip(types.iter().cloned())
                .map(|(value, typ)| codegen_constant_value(value, typ));
            quote!((#(#values),*))
        }
    }
}
