mod automaton;
mod regex_parsing;
use std::collections::HashSet;

#[derive(Debug)]
pub enum VregexError {
    InternalFailure,
    ParsingError,
}

impl From<automaton::AutomatonError> for VregexError {
    fn from(_: automaton::AutomatonError) -> Self {
        VregexError::InternalFailure
    }
}
impl From<String> for VregexError {
    fn from(_: String) -> Self {
        VregexError::ParsingError
    }
}

#[derive(Debug)]
pub struct Vregex {
    pub automaton: automaton::Automaton<char>,
}

impl Vregex {
    // Give the regex here
    pub fn new(regex: &str) -> Self {
        let mut out = Vregex {
            automaton: automaton::Automaton::new(),
        };
        let _ = out.build(regex);
        out
    }

    // Takes s and return if s is in the language defined by the regex given in new()
    pub fn validate(&mut self, s: &str) -> bool {
        self.automaton.init_for_read();
        if s.is_empty() {
            self.automaton.read_empty();
        }
        for c in s.chars() {
            self.automaton.read(&c);
        }
        self.automaton.on_exit()
    }

    fn build(&mut self, s: &str) -> Result<(), VregexError> {
        let reg = regex_parsing::parse_regex(s)?;
        build_automaton(&mut self.automaton, reg)?;
        Ok(())
    }
}

fn build_automaton(autom: &mut automaton::Automaton<char>, reg: regex_parsing::Regex) -> Result<(), VregexError> {
    use crate::vregex::regex_parsing::Regex::*;
    match reg {
        Literal(char) => {
            build_literal(autom, char)?;
            Ok(())
        }
        Concat(left_ex, right_ex) => {
            build_automaton(autom, *left_ex)?;
            let old_exits = autom.get_exit();
            let old_entries = autom.get_entry();
            autom.reset_entry();
            autom.reset_exit();
            build_automaton(autom, *right_ex)?;
            build_concat(autom, &old_entries, &old_exits)?;
            Ok(())
        },
        Plus(left_ex, right_ex) => {
            build_automaton(autom, *left_ex)?;
            build_automaton(autom, *right_ex)?;
            build_plus(autom)?;
            Ok(())
        },
        Star(ex) => {
            build_automaton(autom, *ex)?;
            build_star(autom)?;
            Ok(())
        },
    }
}

fn build_literal(autom: &mut automaton::Automaton<char>, c: char) -> Result<(), VregexError> {
    let nb_added = 2;
    let state_added: Vec<usize> = autom.add_n_state(nb_added);
    autom.add_entry(state_added[0])?;
    autom.add_exit(state_added[1])?;
    autom.add_transition(state_added[0], state_added[1], c)?;
    Ok(())
}

fn build_concat(autom: &mut automaton::Automaton<char>, old_entries: &HashSet<usize>, old_exit: &HashSet<usize>) -> Result<(), VregexError> {
    for o_exit_r in old_exit.iter() {
        for n_entries_r in autom.get_entry().iter() {
            autom.add_eps_transition(*o_exit_r, *n_entries_r)?;
        }
    }
    autom.reset_entry();
    for o_entry_r in old_entries.iter() {
        autom.add_entry(*o_entry_r)?;
    }
    Ok(())
}

fn build_plus(autom: &mut automaton::Automaton<char>) -> Result<(), VregexError> {
    let old_entries = autom.get_entry();
    autom.reset_entry();
    let new_entry = autom.add_state();
    autom.add_entry(new_entry)?;
    for st_r in old_entries.iter() {
        autom.add_eps_transition(new_entry, *st_r)?;
    }
    Ok(())
}

fn build_star(autom: &mut automaton::Automaton<char>) -> Result<(), VregexError> {
    let old_entries = autom.get_entry();
    let old_exits = autom.get_exit();
    autom.reset_entry();
    autom.reset_exit();
    let new_entry = autom.add_state();
    let new_exit = autom.add_state();
    autom.add_entry(new_entry)?;
    autom.add_exit(new_exit)?;
    autom.add_eps_transition(new_entry, new_exit)?;
    autom.add_eps_transition(new_exit, new_entry)?;
    for o_entry_r in old_entries.iter() {
        autom.add_eps_transition(new_entry, *o_entry_r)?;
    }
    for o_exit_r in old_exits.iter() {
        autom.add_eps_transition(*o_exit_r, new_exit)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_literal() {
        let mut vrg = Vregex::new("a");
        assert!(vrg.validate("a").unwrap());
        assert!(!vrg.validate("b").unwrap());
        assert!(!vrg.validate("").unwrap());
    }

    #[test]
    fn test_validate_plus() {
        let mut vrg = Vregex::new("a+b");
        assert!(vrg.validate("a").unwrap());
        assert!(vrg.validate("b").unwrap());
        assert!(!vrg.validate("c").unwrap());
    }

    #[test]
    fn test_validate_concat() {
        let mut vrg = Vregex::new("ab");
        assert!(vrg.validate("ab").unwrap());
        assert!(!vrg.validate("b").unwrap());
        assert!(!vrg.validate("a").unwrap());
        assert!(!vrg.validate("c").unwrap());
        assert!(!vrg.validate("").unwrap());
    }

    #[test]
    fn test_validate_star() {
        let mut vrg = Vregex::new("a*");
        assert!(vrg.validate("a").unwrap());
        assert!(vrg.validate("aaaaaaaaaaa").unwrap());
        assert!(vrg.validate("").unwrap());
        assert!(!vrg.validate("b").unwrap());
        assert!(!vrg.validate("c").unwrap());
    }

    #[test]
    fn test_all() {
        let mut vrg = Vregex::new("((ab)+c)*(z+x)*");
        assert!(vrg.validate("c").unwrap());
        assert!(vrg.validate("ab").unwrap());
        assert!(vrg.validate("abzzzzzxxxxx").unwrap());
        assert!(vrg.validate("ccccccczzzzzxxxxx").unwrap());
        assert!(vrg.validate("ccccccc").unwrap());
        assert!(vrg.validate("ababababzxzxzxzxzxzx").unwrap());
        assert!(vrg.validate("zzxx").unwrap());
        assert!(!vrg.validate("a").unwrap());
        assert!(!vrg.validate("b").unwrap());
        assert!(!vrg.validate("r").unwrap());
    }
}
