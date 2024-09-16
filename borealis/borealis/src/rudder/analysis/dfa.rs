use {
    crate::{
        rudder::{
            statement::{StatementInner, StatementKind},
            Block, Function, Symbol,
        },
        util::arena::{Arena, Ref},
    },
    common::{intern::InternedString, HashMap, HashSet},
};

pub struct SymbolUseAnalysis {
    symbol_uses: HashMap<InternedString, Vec<(Ref<StatementInner>, Ref<Block>)>>,
    symbol_reads: HashMap<InternedString, Vec<(Ref<StatementInner>, Ref<Block>)>>,
    symbol_writes: HashMap<InternedString, Vec<(Ref<StatementInner>, Ref<Block>)>>,
    symbol_blocks: HashMap<InternedString, HashSet<Ref<Block>>>,
}

struct SymbolUseAnalysisBuilder<'f> {
    f: &'f Function,
    inner: SymbolUseAnalysis,
}

impl<'f> SymbolUseAnalysisBuilder<'f> {
    fn analyse(&mut self) {
        self.f.block_iter().collect::<Vec<_>>().into_iter().for_each(|b| {
            let block = b.get(self.f.block_arena());

            block
                .statements()
                .into_iter()
                .filter_map(|stmt| match stmt.get(&block.statement_arena).kind() {
                    StatementKind::ReadVariable { symbol, .. } | StatementKind::WriteVariable { symbol, .. } => {
                        Some((symbol.clone(), stmt))
                    }
                    _ => None,
                })
                .for_each(|(symbol, stmt)| self.insert_use(symbol, stmt, b, &block.statement_arena));
        });
    }

    fn insert_use(
        &mut self,
        symbol: Symbol,
        stmt: Ref<StatementInner>,
        block: Ref<Block>,
        arena: &Arena<StatementInner>,
    ) {
        self.inner
            .symbol_uses
            .entry(symbol.name())
            .and_modify(|u| u.push((stmt, block)))
            .or_insert(vec![(stmt, block)]);

        if let StatementKind::ReadVariable { .. } = stmt.get(arena).kind() {
            self.inner
                .symbol_reads
                .entry(symbol.name())
                .and_modify(|u| u.push((stmt, block)))
                .or_insert(vec![(stmt, block)]);
        }

        if let StatementKind::WriteVariable { .. } = stmt.get(arena).kind() {
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
                u.insert(stmt.get(arena).parent_block());
            })
            .or_insert({
                let mut h = HashSet::default();
                h.insert(stmt.get(arena).parent_block());

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

    pub fn get_symbol_writes(&self, symbol: &Symbol) -> &Vec<(Ref<StatementInner>, Ref<Block>)> {
        self.symbol_writes.get(&symbol.name()).unwrap()
    }

    pub fn get_symbol_reads(&self, symbol: &Symbol) -> &Vec<(Ref<StatementInner>, Ref<Block>)> {
        self.symbol_reads.get(&symbol.name()).unwrap()
    }

    pub fn is_symbol_local(&self, symbol: &Symbol) -> bool {
        self.symbol_blocks.get(&symbol.name()).unwrap().len() == 1
    }
}

pub struct StatementUseAnalysis<'a> {
    arena: &'a mut Arena<Block>,
    block: Ref<Block>,
    stmt_uses: HashMap<Ref<StatementInner>, HashSet<Ref<StatementInner>>>,
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
        for stmt in self.block.get(self.arena).statements() {
            match stmt.get(&self.block.get(&self.arena).statement_arena).kind().clone() {
                StatementKind::WriteVariable { value, .. } => {
                    self.add_use(value, stmt);
                }
                StatementKind::WriteRegister { offset, value } => {
                    self.add_use(offset, stmt);
                    self.add_use(value, stmt);
                }
                StatementKind::ReadRegister { offset, .. } => {
                    self.add_use(offset, stmt);
                }
                StatementKind::ReadMemory { offset, size } => {
                    self.add_use(offset, stmt);
                    self.add_use(size, stmt);
                }
                StatementKind::WriteMemory { offset, value } => {
                    self.add_use(offset, stmt);
                    self.add_use(value, stmt);
                }
                StatementKind::WritePc { value } => {
                    self.add_use(value, stmt);
                }
                StatementKind::BinaryOperation { lhs, rhs, .. } => {
                    self.add_use(lhs, stmt);
                    self.add_use(rhs, stmt);
                }
                StatementKind::UnaryOperation { value, .. } => {
                    self.add_use(value, stmt);
                }
                StatementKind::ShiftOperation { value, amount, .. } => {
                    self.add_use(value, stmt);
                    self.add_use(amount, stmt);
                }
                StatementKind::Call { args, .. } => {
                    for arg in args {
                        self.add_use(arg, stmt);
                    }
                }
                StatementKind::Cast { value, .. } => {
                    self.add_use(value, stmt);
                }
                StatementKind::BitsCast { value, length, .. } => {
                    self.add_use(value, stmt);
                    self.add_use(length, stmt);
                }
                StatementKind::Branch { condition, .. } => {
                    self.add_use(condition, stmt);
                }
                StatementKind::Return { value } => {
                    self.add_use(value, stmt);
                }
                StatementKind::Select {
                    condition,
                    true_value,
                    false_value,
                } => {
                    self.add_use(condition, stmt);
                    self.add_use(true_value, stmt);
                    self.add_use(false_value, stmt);
                }
                StatementKind::BitExtract { value, start, length } => {
                    self.add_use(value, stmt);
                    self.add_use(start, stmt);
                    self.add_use(length, stmt);
                }
                StatementKind::BitInsert {
                    target,
                    source,
                    start,
                    length,
                } => {
                    self.add_use(target, stmt);
                    self.add_use(source, stmt);
                    self.add_use(start, stmt);
                    self.add_use(length, stmt);
                }
                StatementKind::ReadElement { vector, index } => {
                    self.add_use(vector, stmt);
                    self.add_use(index, stmt);
                }
                StatementKind::AssignElement { vector, value, index } => {
                    self.add_use(vector, stmt);
                    self.add_use(value, stmt);
                    self.add_use(index, stmt);
                }
                StatementKind::Assert { condition } => {
                    self.add_use(condition, stmt);
                }
                StatementKind::Panic(value) => {
                    self.add_use(value, stmt);
                }
                StatementKind::CreateBits { value, length } => {
                    self.add_use(value, stmt);
                    self.add_use(length, stmt);
                }

                StatementKind::SizeOf { value } => {
                    self.add_use(value, stmt);
                }

                StatementKind::MatchesUnion { value, .. } => self.add_use(value, stmt),
                StatementKind::UnwrapUnion { value, .. } => self.add_use(value, stmt),

                StatementKind::ReadVariable { .. }
                | StatementKind::ReadPc
                | StatementKind::Jump { .. }
                | StatementKind::PhiNode { .. }
                | StatementKind::Constant { .. }
                | StatementKind::Undefined => {}
                StatementKind::TupleAccess { source, .. } => self.add_use(source, stmt),
                StatementKind::GetFlag { operation, .. } => {
                    self.add_use(operation, stmt);
                }
                StatementKind::CreateTuple(values) => values.into_iter().for_each(|v| self.add_use(v, stmt)),
            }
        }
    }

    fn add_use(&mut self, stmt: Ref<StatementInner>, use_: Ref<StatementInner>) {
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

    pub fn is_dead(&self, stmt: Ref<StatementInner>) -> bool {
        !stmt
            .get(&self.block.get(&self.arena).statement_arena)
            .has_side_effects()
            && !self.has_uses(stmt)
    }

    pub fn has_uses(&self, stmt: Ref<StatementInner>) -> bool {
        self.stmt_uses.contains_key(&stmt)
    }

    pub fn get_uses(&self, stmt: Ref<StatementInner>) -> &HashSet<Ref<StatementInner>> {
        self.stmt_uses.get(&stmt).unwrap()
    }

    pub fn is_used_in_write_var(&self, stmt: Ref<StatementInner>) -> bool {
        if let Some(uses) = self.stmt_uses.get(&stmt) {
            uses.iter().any(|u| {
                matches!(
                    u.get(&self.block.get(&self.arena).statement_arena).kind(),
                    StatementKind::WriteVariable { .. }
                )
            })
        } else {
            false
        }
    }
}
