use {
    crate::rudder::{ConstantValue, Context, PrimitiveTypeClass, StatementKind, Type},
    log::warn,
};

pub fn validate(ctx: &Context) {
    check_constant_value_types(ctx);
}

fn check_constant_value_types(ctx: &Context) {
    for (_, f) in ctx.get_functions() {
        for block in f.entry_block().iter() {
            for stmt in block.statements() {
                if let StatementKind::Constant { typ, value } = stmt.kind() {
                    match value {
                        ConstantValue::UnsignedInteger(_) => match &*typ {
                            Type::Primitive(p) => match p.tc {
                                PrimitiveTypeClass::Void => warn!("cannot use void type class for unsigned integer constant"),
                                PrimitiveTypeClass::Unit => warn!("cannot use unit type class for unsigned integer constant"),
                                PrimitiveTypeClass::UnsignedInteger => {},
                                PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for unsigned integer constant"),
                                PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for unsigned integer constant")
                            },
                            Type::Composite(_) => {
                                warn!("cannot use composite type for unsigned integer constant")
                            }
                            Type::Vector {
                               ..
                            } => warn!("cannot use vector type for unsigned integer constant"),
                            Type::Bundled {.. } => {
                                warn!("cannot use bundled type for unsigned integer constant")
                            }
                        },
                        ConstantValue::SignedInteger(_) => match &*typ {
                            Type::Primitive(p) => match p.tc {
                                PrimitiveTypeClass::Void => warn!("cannot use void type class for signed integer constant"),
                                PrimitiveTypeClass::Unit => warn!("cannot use unit type class for signed integer constant"),
                                PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for signed integer constant {} {} {}", f.name(), block.name(), stmt),
                                PrimitiveTypeClass::SignedInteger => {},
                                PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for signed integer constant")
                            },
                            Type::Composite(_) => {
                                warn!("cannot use composite type for signed integer constant")
                            }
                            Type::Vector {
                                ..
                            } => warn!("cannot use vector type for signed integer constant"),
                            Type::Bundled { .. } => {
                                warn!("cannot use bundled type for signed integer constant")
                            }
                        }
                        ConstantValue::FloatingPoint(_) => match &*typ {
                            Type::Primitive(p) => match p.tc {
                                PrimitiveTypeClass::Void => warn!("cannot use void type class for floating point constant"),
                                PrimitiveTypeClass::Unit => warn!("cannot use unit type class for floating point constant"),
                                PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for floating point constant"),
                                PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for floating point constant"),
                                PrimitiveTypeClass::FloatingPoint => {}
                            },
                            Type::Composite(_) => {
                                warn!("cannot use composite type for floating point constant")
                            }
                            Type::Vector {
                             ..
                            } => warn!("cannot use vector type for floating point constant"),
                            Type::Bundled { .. } => {
                                warn!("cannot use bundled type for floating point constant")
                            }
                        }
                        ConstantValue::Unit => match &*typ {
                            Type::Primitive(p) => match p.tc {
                                PrimitiveTypeClass::Void => warn!("cannot use void type class for unit constant"),
                                PrimitiveTypeClass::Unit => {},
                                PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for unit constant"),
                                PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for unit constant"),
                                PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for unit constant"),
                            },
                            Type::Composite(_) => {
                                warn!("cannot use composite type for unit constant")
                            }
                            Type::Vector {
                              ..
                            } => warn!("cannot use vector type for unit constant"),
                            Type::Bundled {..} => {
                                warn!("cannot use bundled type for unit constant")
                            }
                        }
                    }
                }
            }
        }
    }
}
