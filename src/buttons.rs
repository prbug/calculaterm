use crate::operations::Operator;
use crate::state::CalcState;

#[derive(Debug, Clone, Copy)]
pub enum ButtonType {
    Operator(Operator),
    Calculate,
    Decimal,
    Invert,
    Clear,
    Percent,
    Numeric(u8),
}

pub struct Button {
    button_type: ButtonType,
    pub label: char,
}

impl Button {
    pub fn new(button_type: ButtonType) -> Self {
        let label = match button_type {
            ButtonType::Operator(op) => match op {
                Operator::Add => '+',
                Operator::Subtract => '-',
                Operator::Multiply => 'x',
                Operator::Divide => '/',
            },
            ButtonType::Calculate => '=',
            ButtonType::Decimal => '.',
            ButtonType::Invert => '±',
            ButtonType::Clear => 'C',
            ButtonType::Percent => '%',
            ButtonType::Numeric(n) => u8_to_char(&n),
        };
        Self { button_type, label }
    }

    fn press(self, state: &mut CalcState) {
        match self.button_type {
            ButtonType::Clear => state.clear(),
            ButtonType::Numeric(n) => state.update_input((n + b'0') as char),
            ButtonType::Operator(op) => state.update_operator(op),
            ButtonType::Decimal => state.update_input('.'),
            ButtonType::Calculate => state.calculate(),
            ButtonType::Percent => todo!(),
            ButtonType::Invert => state.invert_input(),
        }
    }
}

pub fn u8_to_char(n: &u8) -> char {
    (n + b'0') as char
}
