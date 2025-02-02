use crate::Action;

pub struct Parser {

}

impl Parser {
    pub fn new() -> Self {
        Self { }
    }

    pub fn parse_as_vec(&self, input: &[u8]) -> Vec<Action> {
        let mut actions = Vec::new();

        for c in input {
            actions.push(Action::Print(*c as char));
        }

        actions
    }
}
