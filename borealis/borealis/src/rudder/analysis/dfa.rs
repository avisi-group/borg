use {
    crate::rudder::{statement::StatementKind, Block, Function, Statement, Symbol},
    common::{intern::InternedString, HashMap, HashSet},
};

pub struct SymbolUseAnalysis {
    f: Function,
    symbol_uses: HashMap<InternedString, Vec<Statement>>,
    symbol_reads: HashMap<InternedString, Vec<Statement>>,
    symbol_writes: HashMap<InternedString, Vec<Statement>>,
    symbol_blocks: HashMap<InternedString, HashSet<Block>>,
}

impl SymbolUseAnalysis {
    pub fn new(f: &Function) -> Self {
        let mut celf = Self {
            f: f.clone(),
            symbol_uses: HashMap::default(),
            symbol_reads: HashMap::default(),
            symbol_writes: HashMap::default(),
            symbol_blocks: HashMap::default(),
        };

        celf.analyse();
        celf
    }

    fn analyse(&mut self) {
        for block in self.f.entry_block().iter() {
            for stmt in block.statements() {
                match stmt.kind() {
                    crate::rudder::StatementKind::ReadVariable { symbol, .. } => {
                        self.insert_use(&symbol, &stmt)
                    }
                    crate::rudder::StatementKind::WriteVariable { symbol, .. } => {
                        self.insert_use(&symbol, &stmt)
                    }
                    _ => {}
                }
            }
        }
    }

    fn insert_use(&mut self, symbol: &Symbol, stmt: &Statement) {
        self.symbol_uses
            .entry(symbol.name())
            .and_modify(|u| u.push(stmt.clone()))
            .or_insert(vec![stmt.clone()]);

        if let StatementKind::ReadVariable { .. } = stmt.kind() {
            self.symbol_reads
                .entry(symbol.name())
                .and_modify(|u| u.push(stmt.clone()))
                .or_insert(vec![stmt.clone()]);
        }

        if let StatementKind::WriteVariable { .. } = stmt.kind() {
            self.symbol_writes
                .entry(symbol.name())
                .and_modify(|u| u.push(stmt.clone()))
                .or_insert(vec![stmt.clone()]);
        }

        self.symbol_blocks
            .entry(symbol.name())
            .and_modify(|u| {
                u.insert(stmt.parent_block().upgrade());
            })
            .or_insert({
                let mut h = HashSet::default();
                h.insert(stmt.parent_block().upgrade());

                h
            });
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

    pub fn get_symbol_writes(&self, symbol: &Symbol) -> &Vec<Statement> {
        self.symbol_writes.get(&symbol.name()).unwrap()
    }

    pub fn get_symbol_reads(&self, symbol: &Symbol) -> &Vec<Statement> {
        self.symbol_reads.get(&symbol.name()).unwrap()
    }

    pub fn is_symbol_local(&self, symbol: &Symbol) -> bool {
        self.symbol_blocks.get(&symbol.name()).unwrap().len() == 1
    }
}

pub struct StatementUseAnalysis {
    block: Block,
    stmt_uses: HashMap<Statement, HashSet<Statement>>,
}

impl StatementUseAnalysis {
    pub fn new(b: &Block) -> Self {
        let mut celf = Self {
            block: b.clone(),
            stmt_uses: HashMap::default(),
        };

        celf.analyse();
        celf
    }

    fn analyse(&mut self) {
        for stmt in self.block.statements() {
            match stmt.kind() {
                StatementKind::WriteVariable { value, .. } => {
                    self.add_use(&value, &stmt);
                }
                StatementKind::WriteRegister { offset, value } => {
                    self.add_use(&offset, &stmt);
                    self.add_use(&value, &stmt);
                }
                StatementKind::ReadRegister { offset, .. } => {
                    self.add_use(&offset, &stmt);
                }
                StatementKind::ReadMemory { offset, size } => {
                    self.add_use(&offset, &stmt);
                    self.add_use(&size, &stmt);
                }
                StatementKind::WriteMemory { offset, value } => {
                    self.add_use(&offset, &stmt);
                    self.add_use(&value, &stmt);
                }
                StatementKind::WritePc { value } => {
                    self.add_use(&value, &stmt);
                }
                StatementKind::BinaryOperation { lhs, rhs, .. } => {
                    self.add_use(&lhs, &stmt);
                    self.add_use(&rhs, &stmt);
                }
                StatementKind::UnaryOperation { value, .. } => {
                    self.add_use(&value, &stmt);
                }
                StatementKind::ShiftOperation { value, amount, .. } => {
                    self.add_use(&value, &stmt);
                    self.add_use(&amount, &stmt);
                }
                StatementKind::Call { args, .. } => {
                    for arg in args {
                        self.add_use(&arg, &stmt);
                    }
                }
                StatementKind::Cast { value, .. } => {
                    self.add_use(&value, &stmt);
                }
                StatementKind::BitsCast { value, length, .. } => {
                    self.add_use(&value, &stmt);
                    self.add_use(&length, &stmt);
                }
                StatementKind::Branch { condition, .. } => {
                    self.add_use(&condition, &stmt);
                }
                StatementKind::Return { value } => {
                    if let Some(value) = value {
                        self.add_use(&value, &stmt);
                    }
                }
                StatementKind::Select {
                    condition,
                    true_value,
                    false_value,
                } => {
                    self.add_use(&condition, &stmt);
                    self.add_use(&true_value, &stmt);
                    self.add_use(&false_value, &stmt);
                }
                StatementKind::BitExtract {
                    value,
                    start,
                    length,
                } => {
                    self.add_use(&value, &stmt);
                    self.add_use(&start, &stmt);
                    self.add_use(&length, &stmt);
                }
                StatementKind::BitInsert {
                    target,
                    source,
                    start,
                    length,
                } => {
                    self.add_use(&target, &stmt);
                    self.add_use(&source, &stmt);
                    self.add_use(&start, &stmt);
                    self.add_use(&length, &stmt);
                }
                StatementKind::ReadElement { vector, index } => {
                    self.add_use(&vector, &stmt);
                    self.add_use(&index, &stmt);
                }
                StatementKind::MutateElement {
                    vector,
                    value,
                    index,
                } => {
                    self.add_use(&vector, &stmt);
                    self.add_use(&value, &stmt);
                    self.add_use(&index, &stmt);
                }
                StatementKind::Assert { condition } => {
                    self.add_use(&condition, &stmt);
                }
                StatementKind::Panic(panic_values) => {
                    for panic_value in panic_values {
                        self.add_use(&panic_value, &stmt);
                    }
                }
                StatementKind::PrintChar(c) => {
                    self.add_use(&c, &stmt);
                }
                StatementKind::CreateBits { value, length } => {
                    self.add_use(&value, &stmt);
                    self.add_use(&length, &stmt);
                }
                StatementKind::CreateProduct { fields, .. } => {
                    for field in fields {
                        self.add_use(&field, &stmt);
                    }
                }
                StatementKind::CreateSum { value, .. } => {
                    self.add_use(&value, &stmt);
                }
                StatementKind::SizeOf { value } => {
                    self.add_use(&value, &stmt);
                }

                StatementKind::MatchesSum { value, .. } => self.add_use(&value, &stmt),
                StatementKind::UnwrapSum { value, .. } => self.add_use(&value, &stmt),
                StatementKind::ExtractField { value, .. } => self.add_use(&value, &stmt),
                StatementKind::UpdateField {
                    original_value,
                    field_value,
                    ..
                } => {
                    self.add_use(&original_value, &stmt);
                    self.add_use(&field_value, &stmt);
                }

                StatementKind::ReadVariable { .. }
                | StatementKind::ReadPc
                | StatementKind::Jump { .. }
                | StatementKind::PhiNode { .. }
                | StatementKind::Constant { .. }
                | StatementKind::Undefined => {}
            }
        }
    }

    fn add_use(&mut self, stmt: &Statement, use_: &Statement) {
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

    pub fn is_dead(&self, stmt: &Statement) -> bool {
        !stmt.has_side_effects() && !self.has_uses(stmt)
    }

    pub fn has_uses(&self, stmt: &Statement) -> bool {
        self.stmt_uses.contains_key(stmt)
    }

    pub fn get_uses(&self, stmt: &Statement) -> &HashSet<Statement> {
        self.stmt_uses.get(stmt).unwrap()
    }

    pub fn is_used_in_write_var(&self, stmt: &Statement) -> bool {
        if let Some(uses) = self.stmt_uses.get(stmt) {
            uses.iter()
                .any(|u| matches!(u.kind(), StatementKind::WriteVariable { .. }))
        } else {
            false
        }
    }
}
