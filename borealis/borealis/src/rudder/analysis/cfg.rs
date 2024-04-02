use std::collections::{HashSet, LinkedList};

use crate::rudder::{Block, Function};
use common::HashMap;

pub struct ControlFlowGraphAnalysis {
    f: Function,
    block_preds: HashMap<Block, Vec<Block>>,
    block_succs: HashMap<Block, Vec<Block>>,
}

impl ControlFlowGraphAnalysis {
    pub fn new(f: Function) -> Self {
        let mut celf = Self {
            f,
            block_preds: HashMap::default(),
            block_succs: HashMap::default(),
        };

        celf.analyse();
        celf
    }

    fn analyse(&mut self) {
        let mut seen_list = HashSet::new();
        let mut work_list = LinkedList::new();
        work_list.push_back(self.f.entry_block());

        self.block_preds
            .insert(work_list.front().unwrap().clone(), Vec::new());

        while !work_list.is_empty() {
            let current = work_list.pop_front().unwrap();
            if seen_list.contains(&current) {
                continue;
            }

            seen_list.insert(current.clone());

            let terminator = current.terminator_statement().unwrap();
            match terminator.kind() {
                crate::rudder::StatementKind::Jump { target } => {
                    self.insert_successor(&current, &target);
                    self.insert_predecessor(&target, &current);

                    work_list.push_back(target.clone());
                }
                crate::rudder::StatementKind::Branch {
                    true_target,
                    false_target,
                    ..
                } => {
                    self.insert_successor(&current, &true_target);
                    self.insert_successor(&current, &false_target);
                    self.insert_predecessor(&true_target, &current);
                    self.insert_predecessor(&false_target, &current);

                    work_list.push_back(true_target.clone());
                    work_list.push_back(false_target.clone());
                }
                crate::rudder::StatementKind::Return { .. } => {
                    self.block_succs.insert(current.clone(), Vec::new());
                }
                _ => panic!("invalid terminator statement for block"),
            }
        }
    }

    fn insert_successor(&mut self, rb: &Block, sb: &Block) {
        self.block_succs
            .entry(rb.clone())
            .and_modify(|e| e.push(sb.clone()))
            .or_insert(vec![sb.clone()]);
    }

    fn insert_predecessor(&mut self, rb: &Block, pb: &Block) {
        self.block_preds
            .entry(rb.clone())
            .and_modify(|e| e.push(pb.clone()))
            .or_insert(vec![pb.clone()]);
    }

    pub fn predecessors_for(&self, block: &Block) -> Option<&Vec<Block>> {
        self.block_preds.get(block)
    }

    pub fn successors_for(&self, block: &Block) -> Option<&Vec<Block>> {
        self.block_succs.get(block)
    }
}
