mod buttons;
mod operations;
mod state;
mod ui;
use color_eyre::Result;

fn main() -> Result<()> {
    let calc_state = state::CalcState::new();
    println!("{:#?}", calc_state);
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = ui::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
