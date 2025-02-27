use common::{
    HashMap, HashSet,
    arena::{Arena, Ref},
    intern::InternedString,
    rudder::{
        block::Block,
        function::{Function, Symbol},
        statement::Statement,
    },
};

pub struct SymbolUseAnalysis {
    symbol_uses: HashMap<InternedString, Vec<(Ref<Statement>, Ref<Block>)>>,
    symbol_reads: HashMap<InternedString, Vec<(Ref<Statement>, Ref<Block>)>>,
    symbol_writes: HashMap<InternedString, Vec<(Ref<Statement>, Ref<Block>)>>,
    symbol_blocks: HashMap<InternedString, HashSet<Ref<Block>>>,
}

struct SymbolUseAnalysisBuilder<'f> {
    f: &'f Function,
    inner: SymbolUseAnalysis,
}

impl<'f> SymbolUseAnalysisBuilder<'f> {
    fn analyse(&mut self) {
        self.f
            .block_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|b| {
                let block = b.get(self.f.arena());

                block
                    .statements()
                    .into_iter()
                    .filter_map(|stmt| match stmt.get(block.arena()) {
                        Statement::ReadVariable { symbol, .. }
                        | Statement::WriteVariable { symbol, .. } => Some((symbol.clone(), stmt)),
                        _ => None,
                    })
                    .for_each(|(symbol, stmt)| self.insert_use(symbol, *stmt, b, block.arena()));
            });
    }

    fn insert_use(
        &mut self,
        symbol: Symbol,
        stmt: Ref<Statement>,
        block: Ref<Block>,
        arena: &Arena<Statement>,
    ) {
        self.inner
            .symbol_uses
            .entry(symbol.name())
            .and_modify(|u| u.push((stmt, block)))
            .or_insert(vec![(stmt, block)]);

        if let Statement::ReadVariable { .. } = stmt.get(arena) {
            self.inner
                .symbol_reads
                .entry(symbol.name())
                .and_modify(|u| u.push((stmt, block)))
                .or_insert(vec![(stmt, block)]);
        }

        if let Statement::WriteVariable { .. } = stmt.get(arena) {
            self.inner
                .symbol_writes
                .entry(symbol.name())
                .and_modify(|u| u.push((stmt, block)))
                .or_insert(vec![(stmt, block)]);
        }

        self.inner
            .symbol_blocks
            .entry(symbol.name())
            .and_modify(|u| {
                u.insert(block);
            })
            .or_insert({
                let mut h = HashSet::default();
                h.insert(block);
                h
            });
    }
}

impl SymbolUseAnalysis {
    pub fn new(f: &Function) -> Self {
        let mut builder = SymbolUseAnalysisBuilder {
            f,
            inner: Self {
                symbol_uses: HashMap::default(),
                symbol_reads: HashMap::default(),
                symbol_writes: HashMap::default(),
                symbol_blocks: HashMap::default(),
            },
        };

        builder.analyse();
        builder.inner
    }

    pub fn is_symbol_dead(&self, symbol: &Symbol) -> bool {
        !self.symbol_blocks.contains_key(&symbol.name())
    }

    pub fn symbol_has_reads(&self, symbol: &Symbol) -> bool {
        self.symbol_reads.contains_key(&symbol.name())
    }

    pub fn symbol_has_writes(&self, symbol: &Symbol) -> bool {
        self.symbol_writes.contains_key(&symbol.name())
    }

    pub fn get_symbol_writes(&self, symbol: &Symbol) -> &Vec<(Ref<Statement>, Ref<Block>)> {
        self.symbol_writes.get(&symbol.name()).unwrap()
    }

    pub fn get_symbol_reads(&self, symbol: &Symbol) -> &Vec<(Ref<Statement>, Ref<Block>)> {
        self.symbol_reads.get(&symbol.name()).unwrap()
    }

    pub fn is_symbol_local(&self, symbol: &Symbol) -> bool {
        self.symbol_blocks.get(&symbol.name()).unwrap().len() == 1
    }
}

pub struct StatementUseAnalysis<'a> {
    arena: &'a mut Arena<Block>,
    block: Ref<Block>,
    stmt_uses: HashMap<Ref<Statement>, HashSet<Ref<Statement>>>,
}

impl<'a> StatementUseAnalysis<'a> {
    pub fn new(arena: &'a mut Arena<Block>, b: Ref<Block>) -> Self {
        let mut celf = Self {
            arena,
            block: b,
            stmt_uses: HashMap::default(),
        };

        celf.analyse();
        celf
    }

    pub fn block_arena(&mut self) -> &mut Arena<Block> {
        self.arena
    }

    fn analyse(&mut self) {
        for stmt in self
            .block
            .get(self.arena)
            .statements()
            .iter()
            .copied()
            .collect::<Vec<_>>()
        {
            match stmt.get(self.block.get(&self.arena).arena()).clone() {
                Statement::WriteVariable { value, .. } => {
                    self.add_use(value, stmt);
                }
                Statement::WriteRegister { offset, value } => {
                    self.add_use(offset, stmt);
                    self.add_use(value, stmt);
                }
                Statement::ReadRegister { offset, .. } => {
                    self.add_use(offset, stmt);
                }
                Statement::ReadMemory {
                    address: offset,
                    size,
                } => {
                    self.add_use(offset, stmt);
                    self.add_use(size, stmt);
                }
                Statement::WriteMemory {
                    address: offset,
                    value,
                } => {
                    self.add_use(offset, stmt);
                    self.add_use(value, stmt);
                }
                Statement::WritePc { value } => {
                    self.add_use(value, stmt);
                }
                Statement::BinaryOperation { lhs, rhs, .. } => {
                    self.add_use(lhs, stmt);
                    self.add_use(rhs, stmt);
                }
                Statement::UnaryOperation { value, .. } => {
                    self.add_use(value, stmt);
                }
                Statement::ShiftOperation { value, amount, .. } => {
                    self.add_use(value, stmt);
                    self.add_use(amount, stmt);
                }
                Statement::Call { args, .. } => {
                    for arg in args {
                        self.add_use(arg, stmt);
                    }
                }
                Statement::Cast { value, .. } => {
                    self.add_use(value, stmt);
                }
                Statement::BitsCast {
                    value,
                    width: length,
                    ..
                } => {
                    self.add_use(value, stmt);
                    self.add_use(length, stmt);
                }
                Statement::Branch { condition, .. } => {
                    self.add_use(condition, stmt);
                }
                Statement::Return { value } => {
                    if let Some(value) = value {
                        self.add_use(value, stmt);
                    }
                }
                Statement::Select {
                    condition,
                    true_value,
                    false_value,
                } => {
                    self.add_use(condition, stmt);
                    self.add_use(true_value, stmt);
                    self.add_use(false_value, stmt);
                }
                Statement::BitExtract {
                    value,
                    start,
                    width: length,
                } => {
                    self.add_use(value, stmt);
                    self.add_use(start, stmt);
                    self.add_use(length, stmt);
                }
                Statement::BitInsert {
                    target,
                    source,
                    start,
                    width: length,
                } => {
                    self.add_use(target, stmt);
                    self.add_use(source, stmt);
                    self.add_use(start, stmt);
                    self.add_use(length, stmt);
                }
                Statement::ReadElement { vector, index } => {
                    self.add_use(vector, stmt);
                    self.add_use(index, stmt);
                }
                Statement::AssignElement {
                    vector,
                    value,
                    index,
                } => {
                    self.add_use(vector, stmt);
                    self.add_use(value, stmt);
                    self.add_use(index, stmt);
                }
                Statement::Assert { condition } => {
                    self.add_use(condition, stmt);
                }
                Statement::Panic(value) => {
                    self.add_use(value, stmt);
                }
                Statement::CreateBits {
                    value,
                    width: length,
                } => {
                    self.add_use(value, stmt);
                    self.add_use(length, stmt);
                }

                Statement::SizeOf { value } => {
                    self.add_use(value, stmt);
                }

                Statement::MatchesUnion { value, .. } => self.add_use(value, stmt),
                Statement::UnwrapUnion { value, .. } => self.add_use(value, stmt),

                Statement::GetFlags { operation } => self.add_use(operation, stmt),

                Statement::ReadVariable { .. }
                | Statement::ReadPc
                | Statement::Jump { .. }
                | Statement::PhiNode { .. }
                | Statement::Constant { .. } => {}

                Statement::TupleAccess { source, .. } => self.add_use(source, stmt),

                Statement::CreateTuple(values) => {
                    values.into_iter().for_each(|v| self.add_use(v, stmt))
                }
                Statement::TernaryOperation { a, b, c, .. } => {
                    self.add_use(a, stmt);
                    self.add_use(b, stmt);
                    self.add_use(c, stmt);
                }
            }
        }
    }

    fn add_use(&mut self, stmt: Ref<Statement>, use_: Ref<Statement>) {
        self.stmt_uses
            .entry(stmt.clone())
            .and_modify(|uses| {
                uses.insert(use_.clone());
            })
            .or_insert({
                let mut uses = HashSet::default();
                uses.insert(use_.clone());
                uses
            });
    }

    pub fn is_dead(&self, stmt: Ref<Statement>) -> bool {
        !stmt
            .get(self.block.get(&self.arena).arena())
            .has_side_effects()
            && !self.has_uses(stmt)
    }

    pub fn has_uses(&self, stmt: Ref<Statement>) -> bool {
        self.stmt_uses.contains_key(&stmt)
    }

    pub fn get_uses(&self, stmt: Ref<Statement>) -> Option<&HashSet<Ref<Statement>>> {
        self.stmt_uses.get(&stmt)
    }

    pub fn is_used_in_write_var(&self, stmt: Ref<Statement>) -> bool {
        if let Some(uses) = self.stmt_uses.get(&stmt) {
            uses.iter().any(|u| {
                matches!(
                    u.get(self.block.get(&self.arena).arena()),
                    Statement::WriteVariable { .. }
                )
            })
        } else {
            false
        }
    }
}
