use crossterm::event::{Event, KeyCode, KeyModifiers};

use super::{Data, State};

pub fn handle_event(event: Event, data: &mut Data) {
    if let Event::Key(key) = event {
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            data.state = State::Quitting;
        } else if key.code == KeyCode::Char(' ') {
            data.computer.perform_cycle();
        }
    }
}