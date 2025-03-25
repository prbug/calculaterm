use crate::buttons::{Button, ButtonType};
use crate::operations::Operator;
use crate::state::CalcState;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    title: String,
    state: CalcState,
    exit: bool,
    selected_button: Option<ButtonType>,
}

impl App {
    pub fn default() -> Self {
        Self {
            title: " Calculaterm ".to_string(),
            state: CalcState::default(),
            exit: false,
            selected_button: None,
        }
    }

    fn grid_location(button_type: &ButtonType) -> Option<(u16, u16)> {
        match button_type {
            ButtonType::Numeric(value) => Some((2, (*value).into())),
            ButtonType::Operator(operator) => match operator {
                Operator::Add => Some((3, 3)),
                Operator::Subtract => Some((2, 3)),
                Operator::Multiply => Some((1, 3)),
                Operator::Divide => Some((0, 3)),
            },
            ButtonType::Clear => Some((0, 0)),
            ButtonType::Invert => Some((0, 1)),
            ButtonType::Percent => Some((0, 2)),
            ButtonType::Decimal => Some((4, 1)),
            ButtonType::Calculate => Some((4, 2)),
        }
    }

    fn key_press_event(label: char) -> event::KeyEvent {
        event::KeyEvent::new(event::KeyCode::Char(label), event::KeyModifiers::NONE)
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let exit_event =
            event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                event::Event::Key(key_event) if key_event == exit_event => {
                    self.exit = true;
                }
                event::Event::Key(key_event) => {
                    self.selected_button = match key_event.code {
                        KeyCode::Enter => Some(ButtonType::Calculate),
                        KeyCode::Char(c) => Button::button_type(c),
                        _ => None,
                    };
                    if let Some(button_type) = self.selected_button {
                        button_type.press(&mut self.state);
                    };
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // Create a layout with a display area and a button grid
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Display area
                Constraint::Min(10),   // Button grid
            ])
            .split(frame.area());
        self.render_display(frame, main_layout[0]);
        self.render_buttons(frame, main_layout[1]);
    }

    //fn handle_events(&mut self) -> Result<()> {
    //    todo!();
    //}

    /// renders the display with the title
    fn render_display(&self, frame: &mut Frame, area: Rect) {
        let display_text = Paragraph::new(Span::styled(
            &self.state.display,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone()),
        );

        frame.render_widget(display_text, area);
    }

    fn render_buttons(&self, frame: &mut Frame, area: Rect) {
        // Define button layout: 5 rows, 4 columns
        let button_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(area);

        // Button rows
        let button_rows = [
            // Row 1: Clear, Invert, Percent, Divide
            vec![
                ButtonType::Clear,
                ButtonType::Invert,
                ButtonType::Percent,
                ButtonType::Operator(Operator::Divide),
            ],
            // Row 2: 7, 8, 9, Multiply
            vec![
                ButtonType::Numeric(7),
                ButtonType::Numeric(8),
                ButtonType::Numeric(9),
                ButtonType::Operator(Operator::Multiply),
            ],
            // Row 3: 4, 5, 6, Subtract
            vec![
                ButtonType::Numeric(4),
                ButtonType::Numeric(5),
                ButtonType::Numeric(6),
                ButtonType::Operator(Operator::Subtract),
            ],
            // Row 4: 1, 2, 3, Add
            vec![
                ButtonType::Numeric(1),
                ButtonType::Numeric(2),
                ButtonType::Numeric(3),
                ButtonType::Operator(Operator::Add),
            ],
            // Row 5: 0, Decimal, Calculate
            vec![
                ButtonType::Numeric(0),
                ButtonType::Decimal,
                ButtonType::Calculate,
            ],
        ];

        // Render each row of buttons
        for (row_index, row_buttons) in button_rows.iter().enumerate() {
            let row_area = button_layout[row_index];
            let row_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(1, row_buttons.len() as u32);
                    row_buttons.len()
                ])
                .split(row_area);

            for (col_index, &button_type) in row_buttons.iter().enumerate() {
                let button = Button::new(button_type);
                let top_padding = (row_layout[col_index].height / 2).saturating_sub(1);
                let button_widget = Paragraph::new(Span::styled(
                    button.label.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .block(
                    Block::bordered()
                        // padding to center the paragraph vertically
                        .padding(Padding::new(
                            0,           // left
                            0,           // right
                            top_padding, // top
                            0,           // bottom
                        )),
                )
                // center the paragraph horizontally
                .centered();

                frame.render_widget(button_widget, row_layout[col_index]);
            }
        }
    }
}
