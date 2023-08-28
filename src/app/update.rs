use crossterm::event::{Event, KeyModifiers, KeyCode};

use super::{App, State};

impl App {
    pub fn update(&mut self, event: Event) {
        match self.state {
            State::Idling => self.update_idling(event),
            _ => (),
        }
    }

    fn update_idling(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                self.state = State::Exiting;
            } else if key.code == KeyCode::Char(' ') {
                self.computer.perform_cycle();
            }
        }
    }
}