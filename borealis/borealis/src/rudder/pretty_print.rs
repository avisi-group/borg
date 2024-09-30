use {
    crate::rudder::analysis::cfg::ControlFlowGraphAnalysis,
    common::rudder::{
        block::Block,
        constant_value::ConstantValue,
        function::{Function, Symbol},
        types::{PrimitiveTypeClass, Type},
        Model,
    },
    itertools::Itertools,
    std::fmt::{Display, Formatter, Result},
};

// impl Display for Function {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let cfg = ControlFlowGraphAnalysis::new(self);

//         self.block_iter().try_for_each(|block| {
//             let preds = cfg
//                 .predecessors_for(block)
//                 .unwrap_or(&vec![])
//                 .iter()
//                 .map(|b| b.index())
//                 .join(", ");

//             let succs = cfg
//                 .successors_for(block)
//                 .unwrap_or(&vec![])
//                 .iter()
//                 .map(|b| b.index())
//                 .join(", ");

//             writeln!(
//                 f,
//                 "  block{}: preds={{{preds}}}, succs={{{succs}}}",
//                 block.index()
//             )?;
//             write!(f, "{}", block.get(self.arena()))
//         })
//     }
// }
