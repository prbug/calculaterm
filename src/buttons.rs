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

impl ButtonType {
    pub fn press(self, state: &mut CalcState) {
        match self {
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

pub struct Button {
    button_type: ButtonType,
    pub label: char,
}

impl Button {
    pub fn new(button_type: ButtonType) -> Self {
        let label = Self::button_char(&button_type);
        Self { button_type, label }
    }

    fn button_char(button_type: &ButtonType) -> char {
        match button_type {
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
            ButtonType::Numeric(n) => u8_to_char(n),
        }
    }

    pub fn button_type(label: char) -> Option<ButtonType> {
        match label {
            '+' => Some(ButtonType::Operator(Operator::Add)),

            '-' => Some(ButtonType::Operator(Operator::Subtract)),
            'x' => Some(ButtonType::Operator(Operator::Multiply)),
            '/' => Some(ButtonType::Operator(Operator::Divide)),
            '=' => Some(ButtonType::Calculate),
            '.' => Some(ButtonType::Decimal),
            '±' => Some(ButtonType::Invert),
            'C' => Some(ButtonType::Clear),
            'c' => Some(ButtonType::Clear),
            '%' => Some(ButtonType::Percent),
            n if n.is_ascii_digit() => Some(ButtonType::Numeric((n as u8) - b'0')),
            _ => None,
        }
    }
}

pub fn u8_to_char(n: &u8) -> char {
    (n + b'0') as char
}
