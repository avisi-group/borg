use {
    crate::{
        rudder::{statement::StatementKind, Block, Function, Model},
        util::arena::Ref,
    },
    common::{intern::InternedString, HashMap, HashSet},
    dot::{GraphWalk, Labeller},
    log::trace,
    std::{collections::VecDeque, io},
};

pub struct ControlFlowGraphAnalysis {
    block_preds: HashMap<Ref<Block>, Vec<Ref<Block>>>,
    block_succs: HashMap<Ref<Block>, Vec<Ref<Block>>>,
}

impl ControlFlowGraphAnalysis {
    pub fn new(f: &Function) -> Self {
        let mut celf = Self {
            block_preds: HashMap::default(),
            block_succs: HashMap::default(),
        };

        celf.analyse(f);
        celf
    }

    fn analyse(&mut self, f: &Function) {
        trace!("analysing function {}", f.name());

        let mut seen_list = HashSet::default();
        let mut work_list = VecDeque::new();
        work_list.push_back(f.entry_block());

        self.block_preds
            .insert(work_list.front().unwrap().clone(), Vec::new());

        while !work_list.is_empty() {
            let current = work_list.pop_front().unwrap();
            if seen_list.contains(&current) {
                continue;
            }

            seen_list.insert(current.clone());

            let current_block = current.get(f.block_arena());
            let terminator = current_block.terminator_statement().unwrap();
            match terminator.get(&current_block.statement_arena).kind() {
                StatementKind::Jump { target } => {
                    self.insert_successor(current, *target);
                    self.insert_predecessor(*target, current);

                    work_list.push_back(*target);
                }
                StatementKind::Branch {
                    true_target,
                    false_target,
                    ..
                } => {
                    self.insert_successor(current, *true_target);
                    self.insert_successor(current, *false_target);
                    self.insert_predecessor(*true_target, current);
                    self.insert_predecessor(*false_target, current);

                    work_list.push_back(*true_target);
                    work_list.push_back(*false_target);
                }
                StatementKind::Return { .. } | StatementKind::Panic { .. } => {
                    self.block_succs.insert(current.clone(), Vec::new());
                }
                _ => panic!("invalid terminator statement for block"),
            }
        }
    }

    fn insert_successor(&mut self, rb: Ref<Block>, sb: Ref<Block>) {
        self.block_succs
            .entry(rb.clone())
            .and_modify(|e| e.push(sb.clone()))
            .or_insert(vec![sb.clone()]);
    }

    fn insert_predecessor(&mut self, rb: Ref<Block>, pb: Ref<Block>) {
        self.block_preds
            .entry(rb.clone())
            .and_modify(|e| e.push(pb.clone()))
            .or_insert(vec![pb.clone()]);
    }

    pub fn predecessors_for(&self, block: Ref<Block>) -> Option<&Vec<Ref<Block>>> {
        self.block_preds.get(&block)
    }

    pub fn successors_for(&self, block: Ref<Block>) -> Option<&Vec<Ref<Block>>> {
        self.block_succs.get(&block)
    }
}

pub struct FunctionCallGraphAnalysis {
    fn_callers: HashMap<InternedString, HashSet<InternedString>>,
    fn_callees: HashMap<InternedString, HashSet<InternedString>>,
}

impl FunctionCallGraphAnalysis {
    pub fn new(ctx: &Model) -> Self {
        let mut selph = Self {
            fn_callers: HashMap::default(),
            fn_callees: HashMap::default(),
        };

        selph.analyse(ctx);

        selph
    }

    fn analyse(&mut self, ctx: &Model) {
        for (fname, f) in ctx.get_functions() {
            assert!(*fname == f.name());

            self.fn_callees.insert(f.name(), HashSet::default());
            self.fn_callers.insert(f.name(), HashSet::default());
        }

        for (_, f) in ctx.get_functions() {
            self.analyse_function(&f);
        }
    }

    fn analyse_function(&mut self, f: &Function) {
        for block_ref in f.block_iter() {
            let block = block_ref.get(f.block_arena());
            let statements = block.statements();
            let call_targets =
                statements
                    .iter()
                    .filter_map(|s| match s.get(&block.statement_arena).kind() {
                        StatementKind::Call { target, .. } => Some(*target),
                        _ => None,
                    });
            // TODO .unique();

            for call_target in call_targets {
                // Callees are functions that *this* function calls.
                self.fn_callees.entry(f.name()).and_modify(|callees| {
                    callees.insert(call_target);
                });

                // Callers are functions that call the target function
                self.fn_callers.entry(call_target).and_modify(|callers| {
                    callers.insert(f.name());
                });
            }
        }
    }

    pub fn get_callers_for(&self, f: InternedString) -> Vec<InternedString> {
        self.fn_callers
            .get(&f)
            .map_or(vec![], |f| f.iter().cloned().collect())
    }

    pub fn get_callees_for(&self, f: InternedString) -> Vec<InternedString> {
        self.fn_callees
            .get(&f)
            .map_or(vec![], |f| f.iter().cloned().collect())
    }

    pub fn to_dot<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        dot::render(self, w)
    }
}

type NodeId = InternedString;
type EdgeId = (NodeId, NodeId);

impl<'ast> Labeller<'ast, NodeId, EdgeId> for FunctionCallGraphAnalysis {
    fn graph_id(&'ast self) -> dot::Id<'ast> {
        dot::Id::new("FCG").unwrap()
    }

    fn node_id(&'ast self, n: &NodeId) -> dot::Id<'ast> {
        dot::Id::new(n.to_string()).unwrap()
    }
}

impl<'ast> GraphWalk<'ast, NodeId, EdgeId> for FunctionCallGraphAnalysis {
    fn nodes(&'ast self) -> dot::Nodes<'ast, NodeId> {
        self.fn_callees
            .keys()
            .cloned()
            .map(|n| crate::codegen::codegen_ident(n).to_string().into())
            .collect::<Vec<InternedString>>()
            .into()
    }

    fn edges(&'ast self) -> dot::Edges<'ast, EdgeId> {
        let edges = Vec::new();

        /*for (caller, callees) in &self.fn_callees {
            for callee in callees {
                edges.push((caller.clone(), callee.clone()));
            }
        }*/

        edges.into()
    }

    fn source(&'ast self, edge: &EdgeId) -> NodeId {
        edge.0
    }

    fn target(&'ast self, edge: &EdgeId) -> NodeId {
        edge.1
    }
}

pub struct FunctionCallGraphPartitioner;

impl FunctionCallGraphPartitioner {
    pub fn new(ctx: &Model) -> Self {
        let fcg = FunctionCallGraphAnalysis::new(ctx);

        let mut selph = Self;
        selph.analyse(fcg);

        selph
    }

    fn analyse(&mut self, _fcg: FunctionCallGraphAnalysis) {
        //
    }
}
