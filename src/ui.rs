use crate::buttons::{Button, ButtonType};
use crate::operations::Operator;
use crate::state::CalcState;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
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

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let exit_event =
            event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL);

        // Enable mouse events
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(terminal.backend_mut(), crossterm::event::EnableMouseCapture)?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                event::Event::Key(key_event) if key_event == exit_event => {
                    self.exit = true;
                }
                event::Event::Key(key_event) => {
                    self.selected_button = match key_event.code {
                        KeyCode::Backspace => Some(ButtonType::Backspace),
                        KeyCode::Enter => Some(ButtonType::Calculate),
                        KeyCode::Char('\n') => Some(ButtonType::Calculate),
                        KeyCode::Char(c) => Button::button_type(c),
                        _ => None,
                    };
                    if let Some(button_type) = self.selected_button {
                        button_type.press(&mut self.state);
                    };
                }
                event::Event::Mouse(mouse_event) => {
                    if let Some(button_type) = self.get_clicked_button(mouse_event) {
                        self.selected_button = Some(button_type);
                        button_type.press(&mut self.state);
                    }
                }
                _ => (),
            }
        }

        // Disable mouse events
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::event::DisableMouseCapture
        )?;

        Ok(())
    }

    fn button_layout() -> [Vec<ButtonType>; 5] {
        [
            vec![
                ButtonType::Clear,
                ButtonType::Invert,
                ButtonType::Percent,
                ButtonType::Operator(Operator::Divide),
            ],
            vec![
                ButtonType::Numeric(7),
                ButtonType::Numeric(8),
                ButtonType::Numeric(9),
                ButtonType::Operator(Operator::Multiply),
            ],
            vec![
                ButtonType::Numeric(4),
                ButtonType::Numeric(5),
                ButtonType::Numeric(6),
                ButtonType::Operator(Operator::Subtract),
            ],
            vec![
                ButtonType::Numeric(1),
                ButtonType::Numeric(2),
                ButtonType::Numeric(3),
                ButtonType::Operator(Operator::Add),
            ],
            vec![
                ButtonType::Numeric(0),
                ButtonType::Decimal,
                ButtonType::Calculate,
            ],
        ]
    }

    /// Determine which button was clicked based on mouse event and button layout
    fn get_clicked_button(&self, mouse_event: event::MouseEvent) -> Option<ButtonType> {
        // Only handle left mouse button clicks
        if mouse_event.kind != event::MouseEventKind::Down(crossterm::event::MouseButton::Left) {
            return None;
        }

        // Button rows (same as in render_buttons method)
        let button_rows = Self::button_layout();

        // Recreate the layout to check button areas
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Display area
                Constraint::Min(10),   // Button grid
            ])
            .split(ratatui::layout::Rect::default()); // Placeholder, will be replaced in draw method

        let button_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(main_layout[1]); // Button grid area

        let mouse_position = Position {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        // Check if mouse click is within button grid
        if !button_layout[0].contains(mouse_position) {
            return None;
        }

        // Determine which row was clicked
        for (row_index, row_area) in button_layout.iter().enumerate() {
            if row_area.contains(mouse_position) {
                let row_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Ratio(
                            1,
                            button_rows[row_index].len() as u32
                        );
                        button_rows[row_index].len()
                    ])
                    .split(*row_area);

                // Check which column was clicked
                for (col_index, &button_type) in button_rows[row_index].iter().enumerate() {
                    if row_layout[col_index].contains(mouse_position) {
                        return Some(button_type);
                    }
                }
                break;
            }
        }

        None
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

        let button_rows = Self::button_layout();

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
                        ))
                        // Apply different background when button is selected
                        .style(
                            Style::default().bg(if self.selected_button == Some(button_type) {
                                Color::DarkGray
                            } else {
                                Color::Reset
                            }),
                        ),
                )
                // center the paragraph horizontally
                .centered();

                frame.render_widget(button_widget, row_layout[col_index]);
            }
        }
    }
}
