use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum StateMachineError {
    InvalidStateId(usize),
}

impl fmt::Display for StateMachineError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StateMachineError::InvalidStateId(id) => write!(f, "No state of id {}", id),
        }
    }
}

impl Error for StateMachineError
{}

#[derive(Debug)]
struct State<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    transitions: HashMap<T, HashSet<usize>>,
    eps_transitions: HashSet<usize>,
}

impl<T> State<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    fn new() -> State<T> {
        State {
            transitions: HashMap::new(),
            eps_transitions: HashSet::new(),
        }
    }

    fn add_transition(&mut self, to: usize, v: T) {
        self.transitions
            .entry(v)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    fn add_eps_transition(&mut self, to: usize) {
        self.eps_transitions
            .insert(to);
    }
}

#[derive(Debug)]
pub struct StateMachine<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    states: Vec<State<T>>,
}

impl<T> StateMachine<T>
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    pub fn new() -> StateMachine<T> {
        StateMachine {
            states: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        self.states.push(State::new());
        self.nb_state()-1
    }

    pub fn add_n_state(&mut self, n: usize) -> Vec<usize> {
        let mut added = Vec::new();
        for _ in 0..n {
            added.push(self.add_state());
        }
        added
    }

    pub fn nb_state(&self) -> usize {
        self.states.len()
    }

    pub fn add_transition(&mut self, from: usize, to: usize, v: T) -> Result<(), StateMachineError> {
        if !self.is_valid_state_id(from) {
            Err(StateMachineError::InvalidStateId(from))
        } else if !self.is_valid_state_id(to) {
            Err(StateMachineError::InvalidStateId(to))
        } else {
            self.states[from].add_transition(to, v);
            Ok(())
        }
    }

    pub fn add_eps_transition(&mut self, from: usize, to: usize) -> Result<(), StateMachineError> {
        if !self.is_valid_state_id(from) {
            Err(StateMachineError::InvalidStateId(from))
        } else if !self.is_valid_state_id(to) {
            Err(StateMachineError::InvalidStateId(to))
        } else {
            self.states[from].add_eps_transition(to);
            Ok(())
        }
    }

    pub fn apply_transition(&self, from: usize, v: &T) -> HashSet<usize> {
        let start = self.apply_eps_transition(from);

        let mut after_symbol = HashSet::new();
        for st in start {
            if let Some(next) = self.states[st].transitions.get(v) {
                after_symbol.extend(next);
            }
        }

        let mut result = HashSet::new();
        for st in after_symbol {
            result.extend(self.apply_eps_transition(st));
        }

        result
    }

    pub fn apply_eps_transition(&self, from: usize) -> HashSet<usize> {
        let mut reachable = HashSet::new();
        let mut stack = vec![from];

        while let Some(st) = stack.pop() {
            if reachable.insert(st) {
                stack.extend(&self.states[st].eps_transitions);
            }
        }
        reachable
    }

    pub fn is_valid_state_id(&self, state_id: usize) -> bool {
        state_id < self.nb_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_state_id() {
        let st: StateMachine<char> = StateMachine::new();
        assert!(!st.is_valid_state_id(1));
    }

    #[test]
    fn test_add_state() {
        let mut st: StateMachine<char> = StateMachine::new();
        assert_eq!(st.nb_state(), 0);
        st.add_state();
        assert_eq!(st.nb_state(), 1);
        st.add_state();
        assert_eq!(st.nb_state(), 2);
    }

    #[test]
    fn test_add_n_state() {
        let mut st: StateMachine<char> = StateMachine::new();
        assert_eq!(st.states.len(), 0);
        let nb_to_add = 100;
        st.add_n_state(nb_to_add);
        assert_eq!(st.states.len(), nb_to_add);
    }

    #[test]
    fn test_add_transition() {
        let mut st: StateMachine<char> = StateMachine::new();
        assert_eq!(st.add_transition(0, 0, ' '), Err(StateMachineError::InvalidStateId(0)));
        st.add_n_state(3);
        assert_eq!(st.add_transition(0, 4, ' '), Err(StateMachineError::InvalidStateId(4)));
        assert_eq!(st.add_transition(4, 0, ' '), Err(StateMachineError::InvalidStateId(4)));
        assert_eq!(st.add_transition(0, 1, ' '), Ok(()));
    }

    #[test]
    fn test_add_eps_transition() {
        let mut st: StateMachine<char> = StateMachine::new();
        assert_eq!(st.add_eps_transition(0, 0), Err(StateMachineError::InvalidStateId(0)));
        st.add_n_state(3);
        assert_eq!(st.add_eps_transition(0, 4), Err(StateMachineError::InvalidStateId(4)));
        assert_eq!(st.add_eps_transition(4, 0), Err(StateMachineError::InvalidStateId(4)));
        assert_eq!(st.add_eps_transition(0, 1), Ok(()));
    }

    fn exemple_state_machine() -> StateMachine<char> {
        let mut st: StateMachine<char> = StateMachine::new();
        let nb_state = 4;
        let _ = st.add_n_state(nb_state);
        let _ = st.add_transition(0, 1, 'a');
        let _ = st.add_transition(0, 2, 'b');
        let _ = st.add_transition(3, 2, 'a');
        let _ = st.add_transition(3, 1, 'b');
        let _ = st.add_transition(3, 1, 'b');
        st
    }

    fn exemple_state_machine_with_eps() -> StateMachine<char> {
        let mut st: StateMachine<char> = StateMachine::new();
        let nb_state = 4;
        let _ = st.add_n_state(nb_state);
        let _ = st.add_transition(0, 1, 'a');
        let _ = st.add_transition(0, 2, 'b');
        let _ = st.add_transition(3, 2, 'a');
        let _ = st.add_transition(3, 1, 'b');
        let _ = st.add_transition(3, 1, 'b');
        let _ = st.add_eps_transition(0, 3);
        let _ = st.add_eps_transition(3, 2);
        let _ = st.add_transition(2, 0, 'a');
        st
    }

    #[test]
    fn test_apply_transition() {
        let st = exemple_state_machine();
        assert!(st.apply_transition(0, &'a').contains(&1));
        assert!(st.apply_transition(3, &'b').contains(&1));
        assert!(st.apply_transition(0, &'b').contains(&2));
        assert!(st.apply_transition(3, &'a').contains(&2));
    }
    
    #[test]
    fn test_apply_transition_with_eps() {
        let st = exemple_state_machine_with_eps();
        assert!(st.apply_transition(0, &'a').contains(&1));
        assert!(st.apply_transition(0, &'a').contains(&2));
        assert!(st.apply_transition(0, &'a').contains(&0));

        assert!(st.apply_transition(0, &'b').contains(&2));
        assert!(st.apply_transition(0, &'b').contains(&1));
    }
}
