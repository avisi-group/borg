use crate::rudder::{Block, Function, Statement, Symbol};
use common::{intern::InternedString, HashMap, HashSet};

pub struct SymbolUseAnalysis {
    f: Function,
    symbol_uses: HashMap<InternedString, Vec<Statement>>,
    symbol_blocks: HashMap<InternedString, HashSet<Block>>,
}

impl SymbolUseAnalysis {
    pub fn new(f: Function) -> Self {
        let mut celf = Self {
            f,
            symbol_uses: HashMap::default(),
            symbol_blocks: HashMap::default(),
        };

        celf.analyse();
        celf
    }

    fn analyse(&mut self) {
        for block in self.f.entry_block().iter() {
            for stmt in block.statements() {
                match stmt.kind() {
                    crate::rudder::StatementKind::ReadVariable { symbol } => {
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

        self.symbol_blocks
            .entry(symbol.name())
            .and_modify(|u| {
                u.insert(stmt.parent().upgrade());
            })
            .or_insert({
                let mut h = HashSet::default();
                h.insert(stmt.parent().upgrade());

                h
            });
    }

    pub fn is_symbol_dead(&self, symbol: &Symbol) -> bool {
        self.symbol_blocks.get(&symbol.name()).is_none()
    }

    pub fn is_symbol_local(&self, symbol: &Symbol) -> bool {
        self.symbol_blocks.get(&symbol.name()).unwrap().len() == 1
    }
}
