use crossterm::event::{Event, KeyModifiers, KeyCode};

use super::{App, State, Component};

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
            } else if key.code == KeyCode::Tab {
                self.next_component();
            } else if key.code == KeyCode::Up {
                match self.selected_comp {
                    Component::PM => {
                        if self.pm_idx > 0 {
                            self.pm_idx -= 1;
                        }
                    },
                    Component::GRX => {
                        if self.grx_idx > 0 {
                            self.grx_idx -= 1;
                        }
                    },
                    Component::K1 => {
                        if self.k1_idx > 0 {
                            self.k1_idx -= 1;
                        }
                    },
                    Component::K2 => {
                        if self.k2_idx > 0 {
                            self.k2_idx -= 1;
                        }
                    },
                    Component::UM => {
                        if self.um_idx > 0 {
                            self.um_idx -= 1;
                        }
                    },
                    _ => (),
                }
            } else if key.code == KeyCode::Down {
                match self.selected_comp {
                    Component::PM => {
                        if self.pm_idx < 255 {
                            self.pm_idx += 1;
                        }
                    },
                    Component::GRX => {
                        if self.grx_idx < 3 {
                            self.grx_idx += 1;
                        }
                    },
                    Component::K1 => {
                        if self.k1_idx < 15 {
                            self.k1_idx += 1;
                        }
                    },
                    Component::K2 => {
                        if self.k2_idx < 3 {
                            self.k2_idx += 1;
                        }
                    },
                    Component::UM => {
                        if self.um_idx < 127 {
                            self.um_idx += 1;
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}