mod computer;
mod widget;

use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use ratatui::prelude::*;

use self::{computer::Computer, widget::CompositeWidget};

#[derive(PartialEq)]
enum State {
    Idling,
    Exiting,
}

struct Data {
    state: State,
    computer: Computer,
}

fn update(event: Event, data: &mut Data) {
    if let Event::Key(key) = event {
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            data.state = State::Exiting;
        } else {
            data.computer.update(key);
        }
    }
}

fn render<B: Backend>(frame: &mut Frame<B>, data: &Data) {
    data.computer.render(frame, frame.size());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let poll_duration = Duration::from_millis(500);

    let mut data = Data {
        state: State::Idling,
        computer: Computer::new(),
    };

    while data.state != State::Exiting {
        terminal.draw(|frame| render(frame, &data))?;
        if event::poll(poll_duration)? {
            update(event::read()?, &mut data);
        }
    } 
    
    Ok(())
}
