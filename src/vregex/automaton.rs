mod state_machine;
use std::collections::HashSet;

#[derive(Debug)]
pub enum AutomatonError {
    InternalFailure,
}

impl From<state_machine::StateMachineError> for AutomatonError {
    fn from(_: state_machine::StateMachineError) -> Self {
        AutomatonError::InternalFailure
    }
}

#[derive(Debug)]
pub struct Automaton<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug + Copy,
{
    nb_state: usize,
    entry: HashSet<usize>,
    exit: HashSet<usize>,
    current_states: HashSet<usize>,
    machine: state_machine::StateMachine<T>,
}

impl<T> Automaton<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug + Copy,
{
    pub fn new() -> Automaton<T> {
        Automaton {
            nb_state: 0,
            entry: HashSet::new(),
            exit: HashSet::new(),
            current_states: HashSet::new(),
            machine: state_machine::StateMachine::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        self.nb_state += 1;
        self.machine.add_state()
    }

    pub fn add_n_state(&mut self, n: usize) -> Vec<usize> {
        self.nb_state += n;
        self.machine.add_n_state(n)
    }

    pub fn add_entry(&mut self, i: usize) -> Result<(), AutomatonError> {
        if !self.machine.is_valid_state_id(i) {
            Err(AutomatonError::InternalFailure)
        } else {
            self.entry.insert(i);
            Ok(())
        }
    }

    pub fn reset_entry(&mut self) {
        self.entry = HashSet::new();
    }

    pub fn get_entry(&self) -> HashSet<usize> {
        self.entry.clone()
    }

    pub fn add_exit(&mut self, i: usize) -> Result<(), AutomatonError> {
        if !self.machine.is_valid_state_id(i) {
            Err(AutomatonError::InternalFailure)
        } else {
            self.exit.insert(i);
            Ok(())
        }
    }

    pub fn get_exit(&self) -> HashSet<usize> {
        self.exit.clone()
    }

    pub fn reset_exit(&mut self) {
        self.exit = HashSet::new();
    }

    pub fn add_transition(&mut self, from: usize, to: usize, v: T) -> Result<(), AutomatonError> {
        self.machine.add_transition(from, to, v)?;
        Ok(())
    }

    pub fn add_eps_transition(&mut self, from: usize, to: usize) -> Result<(), AutomatonError> {
        self.machine.add_eps_transition(from, to)?;
        Ok(())
    }

    pub fn init_for_read(&mut self) {
        self.current_states = self.entry.clone();
    }

    pub fn read(&mut self, v: &T) {
        let next_states: HashSet<usize> = self.current_states
            .iter()
            .flat_map(|st| {
                self.machine.apply_transition(*st, v)
            })
            .collect();
        self.current_states = next_states;
    }

    pub fn read_empty(&mut self) {
        let next_states: HashSet<usize> = self.current_states
            .iter()
            .flat_map(|st| {
                self.machine.apply_eps_transition(*st)
            })
            .collect();
        self.current_states = next_states;
    }

    pub fn on_exit(&self) -> bool {
        !self.current_states
            .intersection(&self.exit)
            .collect::<HashSet<_>>()
            .is_empty()
    }
}
