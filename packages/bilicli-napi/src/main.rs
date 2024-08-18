use std::io::stdout;

use bilicli_napi::{api::get_room_info, app::App, init_panic_hook};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_panic_hook();

    let room_id = 30897007;
    let mut app = App::new(room_id);

    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let info = get_room_info(room_id).await.unwrap();
    app.update_info(info);
    app.run(&mut terminal).await.unwrap();

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
