use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum EditType {
    INSERT,
    DELETE,
    SAME,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edit {
    pub old_index: usize,
    pub new_index: usize,
    pub edit_type: EditType,
    pub tokens: Vec<Token>,
}

impl Edit {
    pub fn new(old_index: usize, new_index: usize, edit_type: EditType, token: Token) -> Self {
        Edit {
            old_index,
            new_index,
            edit_type,
            tokens: vec![token],
        }
    }

    pub fn new_with_tokens(
        old_index: usize,
        new_index: usize,
        edit_type: EditType,
        tokens: Vec<Token>,
    ) -> Self {
        Edit {
            old_index,
            new_index,
            edit_type,
            tokens,
        }
    }

    pub fn append_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
