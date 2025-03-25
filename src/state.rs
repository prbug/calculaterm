use crate::operations::Operator;

#[derive(Debug, Default)]
pub struct CalcState {
    pub display: String,
    pub input: Option<String>,
    pub result: Option<f64>,
    pub error: Option<String>,
    pub operator: Option<Operator>,
    pub operand: Option<f64>,
}

impl CalcState {
    pub fn new() -> CalcState {
        CalcState {
            display: "0".to_string(),
            input: None,
            result: None,
            error: None,
            operator: None,
            operand: None,
        }
    }

    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.input = None;
        self.result = None;
        self.error = None;
        self.operator = None;
        self.operand = None;
    }

    fn update_display(&mut self) {
        // Prioritize error state
        if let Some(error) = &self.error {
            self.display = error.to_string();
            return;
        }

        // Then check result state
        if let Some(result) = self.result {
            // Round to handle floating-point precision
            self.display = format!("{:.10}", result)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
            return;
        }

        // Then check input state
        match self.get_input_value() {
            Some(value) => {
                self.display = format!("{:.10}", value)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
            None => self.display = "0".to_string(),
        }
    }

    pub fn update_input(&mut self, n: char) {
        if self.result.is_some() || self.error.is_some() {
            self.clear();
        }
        match self.input {
            Some(ref mut input) => input.push(n),
            None => self.input = Some(n.to_string()),
        }
        self.update_display();
    }

    fn get_input_value(&self) -> Option<f64> {
        match self.input {
            Some(ref input) => {
                if input == "." {
                    Some(0.0)
                } else {
                    input.parse().ok()
                }
            }
            None => None,
        }
    }

    pub fn invert_input(&mut self) {
        if let Some(input) = self.get_input_value() {
            self.input = Some((-input).to_string());
            self.update_display();
        }
    }

    /// Update the operator and move the input value to the operand
    pub fn update_operator(&mut self, op: Operator) {
        if self.operand.is_none() {
            if let Some(value) = self.get_input_value() {
                self.operand = Some(value);
                self.input = None;
            }
        }
        self.operator = Some(op);
    }

    /// calculate the result based on the current operator and operand but an error will
    /// be displayed if the conditions for calculation are not met.
    pub fn calculate(&mut self) {
        if self.operand.is_some() && self.operator.is_some() {
            let target_value = match self.get_input_value() {
                Some(value) => Some(value),
                None => self.result,
            };
            match (self.operand, self.operator, target_value) {
                (Some(operand), Some(operator), Some(value)) => {
                    self.result = match operator {
                        Operator::Add => Some(operand + value),
                        Operator::Subtract => Some(operand - value),
                        Operator::Multiply => Some(operand * value),
                        Operator::Divide => {
                            if value == 0.0 {
                                self.clear();
                                self.error = Some("Divide by Zero".to_string());
                                None
                            } else {
                                Some(operand / value)
                            }
                        }
                    };
                    self.input = None;
                }
                _ => {
                    self.error = Some("Calculation Error".to_string());
                    self.result = None;
                }
            }
        }
    }
}
