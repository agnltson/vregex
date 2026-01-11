use std::collections::HashMap;

#[derive(Debug)]
enum StateType {
    Entry,
    End,
    Regular,
}

#[derive(Debug)]
struct State<'a> {
    state_type: StateType,
    transitions: HashMap<char, &'a State<'a>>,
}

impl<'a> State<'a> {
    fn new() -> State<'a> {
        State {
            state_type: StateType::Regular,
            transitions: HashMap::new(),
        }
    }

    fn new_entry() -> State<'a> {
        State {
            state_type: StateType::Entry,
            transitions: HashMap::new(),
        }
    }

    fn new_end() -> State<'a> {
        State {
            state_type: StateType::End,
            transitions: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct StateMachine<'a> {
    entry_idx: usize,
    end_idx: usize,
    states: Vec<State<'a>>,
}

impl<'a> StateMachine<'a> {
    pub fn new() -> StateMachine<'a> {
        let mut st = StateMachine {
            entry_idx: 0,
            end_idx: 1,
            states: Vec::new(),
        };
        st.states.push(State::new_entry());
        st.states.push(State::new_end());
        st
    }

    pub fn add_state(&mut self) {
        self.states.push(State::new());
    }
}
