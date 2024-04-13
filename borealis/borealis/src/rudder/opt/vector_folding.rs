use {
    crate::rudder::{
        BinaryOperationKind, Block, ConstantValue, Function, StatementBuilder, StatementKind, Type,
    },
    log::trace,
    std::rc::Rc,
};

pub fn run(f: Function) -> bool {
    let mut changed = false;
    // for block in f.entry_block().iter() {
    //     changed |= run_on_block(&block);
    // }

    changed
}

// Replace vector access on registers and locals with adding to the indices and offset respectively
// fn run_on_block(block: &Block) -> bool {
//     for stmt in block.statements() {
//         if let StatementKind::ReadElement { vector, index } = stmt.kind() {
//             if let StatementKind::ReadRegister { typ, offset } = composite.kind() {
//                 trace!("possible optimisation candidate");

//                 let field_type = stmt.typ();

//                 // replace read-field with a read-reg with a new computed offset

//                 // 1: some statement that produces a register offset
//                 // 2: read-reg 1
//                 // > 3: read-field 2.Y
//                 // ->
//                 // 1: some statement that produces a register offset
//                 // 2: read-reg 1

//                 // 3: const #byte offset of field
//                 // 4: add 1 3
//                 // > 5: read-reg 4

//                 let byte_offset = typ.byte_offset(field).unwrap();

//                 let mut builder = StatementBuilder::new(block.weak());
//                 let byte_offset_of_field = builder.build(StatementKind::Constant {
//                     typ: Rc::new(Type::u64()),
//                     value: ConstantValue::UnsignedInteger(byte_offset),
//                 });
//                 let new_offset = builder.build(StatementKind::BinaryOperation {
//                     kind: BinaryOperationKind::Add,
//                     lhs: offset.clone(),
//                     rhs: byte_offset_of_field.clone(),
//                 });

//                 block.insert_statement_before(&stmt, new_offset.clone());
//                 block.insert_statement_before(&new_offset, byte_offset_of_field);

//                 stmt.replace_kind(StatementKind::ReadRegister {
//                     typ: field_type,
//                     offset: new_offset,
//                 });

//                 return true;
//             }
//         }
//     }

//     false
// }
