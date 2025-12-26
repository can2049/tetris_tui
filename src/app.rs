use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{game::GameState, input, ui::render};

const DEFAULT_TICK_RATE_MS: u64 = 500;

pub struct App {
    state: GameState,
    tick_rate: Duration,
    last_tick: Instant,
    confirm_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            tick_rate: Duration::from_millis(DEFAULT_TICK_RATE_MS),
            last_tick: Instant::now(),
            confirm_exit: false,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.main_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn main_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|frame| render::render(frame, &self.state, self.confirm_exit))?;

            let timeout = self.tick_rate.saturating_sub(self.last_tick.elapsed());
            if event::poll(timeout)? {
                let evt = event::read()?;
                if self.handle_event(evt) {
                    break;
                }
            }

            if self.last_tick.elapsed() >= self.tick_rate {
                self.on_tick();
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) => {
                self.handle_key_event(key)
            }
            Event::Resize(_, _) => false,
            _ => false,
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return true;
        }

        if self.confirm_exit {
            return match key.code {
                KeyCode::Char('y') | KeyCode::Enter => true,
                KeyCode::Char('n') | KeyCode::Esc => {
                    self.confirm_exit = false;
                    false
                }
                _ => false,
            };
        }

        match key.code {
            KeyCode::Char('q') => {
                if self.state.game_over {
                    return true;
                }
                self.confirm_exit = true;
            }
            _ => input::handle_key(&mut self.state, key),
        }

        false
    }

    fn on_tick(&mut self) {
        if !self.confirm_exit {
            self.state.tick();
        }
        self.last_tick = Instant::now();
    }
}
