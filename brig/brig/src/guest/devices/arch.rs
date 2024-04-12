/*pub trait Architecture {
    type State;
    fn initial_state() -> Self::State;
}

pub trait Core {
    type State;
    fn initial_state() -> <Self as Core>::State;
}

pub trait ExecutionEngine {
    fn step(&mut self, state: &mut <ExecutionEngine as Core>::State);
}

struct Aarch64Archicture {
    cores: [Aarch64Core; 4],
}

struct Aarch64Core {}
*/
