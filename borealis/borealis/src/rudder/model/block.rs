use {
    crate::{
        rudder::model::statement::{Statement, StatementKind},
        util::arena::{Arena, Ref},
    },
    common::HashSet,
};

#[derive(Clone, Debug)]
pub struct Block {
    pub(crate) statement_arena: Arena<Statement>,
    pub(crate) statements: Vec<Ref<Statement>>,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_index(&mut self) {
        self.statements
            .iter()
            .enumerate()
            .for_each(|(statement_index, stmt)| {
                stmt.get_mut(&mut self.statement_arena)
                    .update_names(format!("s{statement_index}").into());
            });
    }

    pub fn statements(&self) -> Vec<Ref<Statement>> {
        self.statements.clone()
    }

    pub fn terminator_statement(&self) -> Option<Ref<Statement>> {
        self.statements.last().cloned()
    }

    pub fn set_statements<I: Iterator<Item = Ref<Statement>>>(&mut self, statements: I) {
        self.statements = statements.collect();
    }

    pub fn extend_statements<I: Iterator<Item = Ref<Statement>>>(&mut self, stmts: I) {
        self.statements.extend(stmts)
    }

    pub(crate) fn index_of_statement(&self, reference: Ref<Statement>) -> usize {
        self.statements
            .iter()
            .enumerate()
            .find(|(_, candidate)| **candidate == reference)
            .unwrap()
            .0
    }

    pub fn insert_statement_before(&mut self, reference: Ref<Statement>, new: Ref<Statement>) {
        let index = self.index_of_statement(reference);
        self.statements.insert(index, new);
    }

    pub fn append_statement(&mut self, new: Ref<Statement>) {
        self.statements.push(new);
    }

    pub fn kill_statement(&mut self, stmt: Ref<Statement>) {
        //assert!(Rc::ptr_eq()

        let index = self.index_of_statement(stmt);

        self.statements.remove(index);
    }

    pub fn targets(&self) -> Vec<Ref<Block>> {
        match self
            .terminator_statement()
            .unwrap()
            .get(&self.statement_arena)
            .kind()
        {
            StatementKind::Jump { target } => vec![*target],
            StatementKind::Branch {
                true_target,
                false_target,
                ..
            } => vec![*true_target, *false_target],
            StatementKind::Return { .. } | StatementKind::Panic(_) => {
                vec![]
            }
            k => panic!("invalid terminator for block: {k:?}"),
        }
    }

    pub fn size(&self) -> usize {
        self.statements().len()
    }

    pub fn arena_mut(&mut self) -> &mut Arena<Statement> {
        &mut self.statement_arena
    }

    pub fn arena(&self) -> &Arena<Statement> {
        &self.statement_arena
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            statements: Vec::new(),
            statement_arena: Arena::new(),
        }
    }
}

pub struct BlockIterator<'arena> {
    pub(crate) visited: HashSet<Ref<Block>>,
    pub(crate) remaining: Vec<Ref<Block>>,
    pub(crate) arena: &'arena Arena<Block>,
}

impl<'arena> BlockIterator<'arena> {
    pub(crate) fn new(arena: &'arena Arena<Block>, start_block: Ref<Block>) -> Self {
        Self {
            visited: HashSet::default(),
            remaining: vec![start_block],
            arena,
        }
    }
}

impl<'arena> Iterator for BlockIterator<'arena> {
    type Item = Ref<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            let Some(current) = self.remaining.pop() else {
                // if remaining is empty, all blocks have been visited
                return None;
            };

            // if we've already visited the node, get the next one
            if self.visited.contains(&current) {
                continue;
            } else {
                // otherwise return it
                break current;
            }
        };

        // mark current node as processed
        self.visited.insert(current.clone());

        // push children to visit
        self.remaining.extend(current.get(&self.arena).targets());

        Some(current)
    }
}
