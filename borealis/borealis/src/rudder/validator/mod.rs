use log::warn;

use super::{Context, StatementKind};

pub fn validate(ctx: &Context) {
    check_constant_value_types(ctx);
}

fn check_constant_value_types(ctx: &Context) {
    for (_, f) in ctx.get_functions() {
        for block in f.entry_block().iter() {
            for stmt in block.statements() {
                if let StatementKind::Constant { typ, value } = stmt.kind() {
                    match value {
                        super::ConstantValue::UnsignedInteger(_) => match &*typ {
                            super::Type::Primitive(p) => match p.tc {
                                super::PrimitiveTypeClass::Void => warn!("cannot use void type class for unsigned integer constant"),
                                super::PrimitiveTypeClass::Unit => warn!("cannot use unit type class for unsigned integer constant"),
                                super::PrimitiveTypeClass::UnsignedInteger => {},
                                super::PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for unsigned integer constant"),
                                super::PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for unsigned integer constant")
                            },
                            super::Type::Composite(_) => {
                                warn!("cannot use composite type for unsigned integer constant")
                            }
                            super::Type::Vector {
                                element_count,
                                element_type,
                            } => warn!("cannot use vector type for unsigned integer constant"),
                            super::Type::Bundled { value, len } => {
                                warn!("cannot use bundled type for unsigned integer constant")
                            }
                        },
                        super::ConstantValue::SignedInteger(_) => match &*typ {
                            super::Type::Primitive(p) => match p.tc {
                                super::PrimitiveTypeClass::Void => warn!("cannot use void type class for signed integer constant"),
                                super::PrimitiveTypeClass::Unit => warn!("cannot use unit type class for signed integer constant"),
                                super::PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for signed integer constant {} {} {}", f.name(), block.name(), stmt),
                                super::PrimitiveTypeClass::SignedInteger => {},
                                super::PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for signed integer constant")
                            },
                            super::Type::Composite(_) => {
                                warn!("cannot use composite type for signed integer constant")
                            }
                            super::Type::Vector {
                                element_count,
                                element_type,
                            } => warn!("cannot use vector type for signed integer constant"),
                            super::Type::Bundled { value, len } => {
                                warn!("cannot use bundled type for signed integer constant")
                            }
                        }
                        super::ConstantValue::FloatingPoint(_) => match &*typ {
                            super::Type::Primitive(p) => match p.tc {
                                super::PrimitiveTypeClass::Void => warn!("cannot use void type class for floating point constant"),
                                super::PrimitiveTypeClass::Unit => warn!("cannot use unit type class for floating point constant"),
                                super::PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for floating point constant"),
                                super::PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for floating point constant"),
                                super::PrimitiveTypeClass::FloatingPoint => {}
                            },
                            super::Type::Composite(_) => {
                                warn!("cannot use composite type for floating point constant")
                            }
                            super::Type::Vector {
                                element_count,
                                element_type,
                            } => warn!("cannot use vector type for floating point constant"),
                            super::Type::Bundled { value, len } => {
                                warn!("cannot use bundled type for floating point constant")
                            }
                        }
                        super::ConstantValue::Unit => match &*typ {
                            super::Type::Primitive(p) => match p.tc {
                                super::PrimitiveTypeClass::Void => warn!("cannot use void type class for unit constant"),
                                super::PrimitiveTypeClass::Unit => {},
                                super::PrimitiveTypeClass::UnsignedInteger => warn!("cannot use unsigned integer type class for unit constant"),
                                super::PrimitiveTypeClass::SignedInteger => warn!("cannot use signed integer type class for unit constant"),
                                super::PrimitiveTypeClass::FloatingPoint => warn!("cannot use floating point type class for unit constant"),
                            },
                            super::Type::Composite(_) => {
                                warn!("cannot use composite type for unit constant")
                            }
                            super::Type::Vector {
                                element_count,
                                element_type,
                            } => warn!("cannot use vector type for unit constant"),
                            super::Type::Bundled { value, len } => {
                                warn!("cannot use bundled type for unit constant")
                            }
                        }
                    }
                }
            }
        }
    }
}
