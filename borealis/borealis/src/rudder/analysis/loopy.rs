use {crate::rudder::Function, common::HashSet, std::collections::VecDeque};

pub struct LoopAnalysis {
    contains_loop: bool,
}

impl LoopAnalysis {
    pub fn new(f: &Function) -> Self {
        let mut selph = Self { contains_loop: false };

        selph.analyse(f);

        selph
    }

    fn analyse(&mut self, f: &Function) {
        let mut work_list = VecDeque::new();
        let mut seen_list = HashSet::default();

        work_list.push_back(f.entry_block());

        while !work_list.is_empty() {
            let current = work_list.pop_front().unwrap();

            if seen_list.contains(&current) {
                self.contains_loop = true;
                break;
            }

            work_list.extend(current.get(f.block_arena()).targets());
            seen_list.insert(current);
        }
    }

    pub fn contains_loop(&self) -> bool {
        self.contains_loop
    }
}
