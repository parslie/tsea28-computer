use ratatui::{prelude::*, widgets::*};

use super::{App, Component};

impl App {
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4+4+16+2),
                Constraint::Length(8+2),
                Constraint::Length((3+2+7+2)*2),
                Constraint::Length(4+3+25+2),
                Constraint::Min(0),
            ])
            .split(frame.size());

        self.render_column_1(frame, layout[0]);
        self.render_column_2(frame, layout[1]);
        self.render_column_3(frame, layout[2]);
        self.render_column_4(frame, layout[3]);
    }

    // Column rendering

    fn render_column_1<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1+2),
                Constraint::Max(256+2),
                Constraint::Length(1+2),
                Constraint::Length(1+2),
                Constraint::Length(4+2),
                Constraint::Min(0),
            ])
            .split(area);
    
        let row_1_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column_layout[0]);
    
        self.render_asr(frame, row_1_layout[0]);
        self.render_pc(frame, row_1_layout[1]);
        self.render_pm(frame, column_layout[1]);
        self.render_ar(frame, column_layout[2]);
        self.render_hr(frame, column_layout[3]);
        self.render_grx(frame, column_layout[4]);
    }
    
    fn render_column_2<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1+2),
                Constraint::Length(1+2),
                Constraint::Min(0),
            ])
            .split(area);
    
        self.render_flags(frame, column_layout[0]);
        self.render_lc(frame, column_layout[1]);
    }
    
    fn render_column_3<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1+2),
                Constraint::Max(1+2+16+2),
                Constraint::Min(0),
            ])
            .split(area);
    
        let row_2_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(column_layout[1]);
    
        let column_k1_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1+2),
                Constraint::Max(16+2),
                Constraint::Min(0),
            ])
            .split(row_2_layout[0]);
    
        let column_k2_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1+2),
                Constraint::Max(4+2),
                Constraint::Min(0),
            ])
            .split(row_2_layout[1]);
    
        self.render_ir(frame, column_layout[0]);
        self.render_upc(frame, column_k1_layout[0]);
        self.render_k1(frame, column_k1_layout[1]);
        self.render_supc(frame, column_k2_layout[0]);
        self.render_k2(frame, column_k2_layout[1]);
    }
    
    fn render_column_4<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(128+2),
                Constraint::Min(0),
            ])
            .split(area);
    
        self.render_um(frame, column_layout[0]);
    }
    
    // Component rendering
    
    fn render_asr<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::ASR = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" ASR : 111 ");
        let paragraph = Paragraph::new(format!("{:0>8b}", self.computer.address_reg))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_pc<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::PC = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" PC : 011 ");
        let paragraph = Paragraph::new(format!("{:0>8b}", self.computer.program_counter))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_ar<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::AR = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" AR : 100 ");
        let paragraph = Paragraph::new(format!("{:0>16b}", self.computer.accumulator_reg))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_hr<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::HR = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" HR : 101 ");
        let paragraph = Paragraph::new(format!("{:0>16b}", self.computer.accumulator_reg))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_flags<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::FLAGS = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" ZNCOL ");
        let paragraph = Paragraph::new(format!("{:0>5b}", self.computer.flags))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_upc<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::UPC = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" µPC ");
        let paragraph = Paragraph::new(format!("{:0>7b}", self.computer.micro_counter))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_supc<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::SUPC = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" SµPC ");
        let paragraph = Paragraph::new(format!("{:0>7b}", self.computer.saved_micro_counter))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_lc<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::LC = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" LC ");
        let paragraph = Paragraph::new(format!("{:0>8b}", self.computer.loop_counter))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_ir<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let para_style = if let Component::IR = self.selected_comp {
            Style::new().fg(Color::Black).bg(Color::White)
        } else {
            Style::new()
        }; 

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" IR : 001 ");
        let paragraph = Paragraph::new(format!("{:0>16b}", self.computer.instruction_reg))
            .alignment(Alignment::Right)
            .style(para_style);
    
        frame.render_widget(paragraph, block.inner(area));
        frame.render_widget(block, area);
    }
    
    fn render_k1<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" K1 ");
    
        let inner_area = block.inner(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(7),
            ])
            .split(inner_area);
    
        let mut min_idx = self.k1_offset as usize;
        let mut max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, 2usize.pow(4));

        if self.k1_idx < min_idx {
            let diff = min_idx - self.k1_idx;
            self.k1_offset -= diff;
            min_idx -= diff;
            max_idx -= diff;
        } else if self.k1_idx > max_idx - 1 {
            let diff = self.k1_idx - (max_idx - 1);
            self.k1_offset += diff;
            min_idx += diff;
            max_idx += diff;
        }
    
        let mut key_lines = Vec::new();
        let mut value_lines = Vec::new();
    
        for idx in min_idx..max_idx {
            let line_style = if self.selected_comp == Component::K1 && idx == self.k1_idx {
                Style::new().fg(Color::Black).bg(Color::White)
            } else {
                Style::new()
            };

            key_lines.push(Line::styled(format!("0x{:0>1x}", idx), line_style));
            value_lines.push(Line::styled(format!("{:0>7b}", self.computer.k1[idx]), line_style));
        }
    
        let key_paragraph = Paragraph::new(key_lines)
            .alignment(Alignment::Left);
        let value_paragraph = Paragraph::new(value_lines)
            .alignment(Alignment::Right);
    
        frame.render_widget(key_paragraph, inner_layout[0]);
        frame.render_widget(value_paragraph, inner_layout[1]);
        frame.render_widget(block, area);
    }
    
    fn render_k2<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" K2 ");
    
        let inner_area = block.inner(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(7),
            ])
            .split(inner_area);
    
        let mut min_idx = self.k2_offset as usize;
        let mut max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, 4);

        if self.k2_idx < min_idx {
            let diff = min_idx - self.k2_idx;
            self.k2_offset -= diff;
            min_idx -= diff;
            max_idx -= diff;
        } else if self.k2_idx > max_idx - 1 {
            let diff = self.k2_idx - (max_idx - 1);
            self.k2_offset += diff;
            min_idx += diff;
            max_idx += diff;
        }
    
        let mut key_lines = Vec::new();
        let mut value_lines = Vec::new();
    
        for idx in min_idx..max_idx {
            let line_style = if self.selected_comp == Component::K2 && idx == self.k2_idx {
                Style::new().fg(Color::Black).bg(Color::White)
            } else {
                Style::new()
            };

            key_lines.push(Line::styled(format!("0x{:0>1x}", idx), line_style));
            value_lines.push(Line::styled(format!("{:0>7b}", self.computer.k2[idx]), line_style));
        }
    
        let key_paragraph = Paragraph::new(key_lines)
            .alignment(Alignment::Left);
        let value_paragraph = Paragraph::new(value_lines)
            .alignment(Alignment::Right);
    
        frame.render_widget(key_paragraph, inner_layout[0]);
        frame.render_widget(value_paragraph, inner_layout[1]);
        frame.render_widget(block, area);
    }
    
    fn render_grx<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" GRx : 110 ");
    
        let inner_area = block.inner(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(16),
            ])
            .split(inner_area);
    
        let mut min_idx = self.grx_offset as usize;
        let mut  max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, 4);

        if self.grx_idx < min_idx {
            let diff = min_idx - self.grx_idx;
            self.grx_offset -= diff;
            min_idx -= diff;
            max_idx -= diff;
        } else if self.grx_idx > max_idx - 1 {
            let diff = self.grx_idx - (max_idx - 1);
            self.grx_offset += diff;
            min_idx += diff;
            max_idx += diff;
        }
    
        let mut key_lines = Vec::new();
        let mut value_lines = Vec::new();
    
        for idx in min_idx..max_idx {
            let line_style = if self.selected_comp == Component::GRX && idx == self.grx_idx {
                Style::new().fg(Color::Black).bg(Color::White)
            } else {
                Style::new()
            };

            key_lines.push(Line::styled(format!("0x{:0>1x}", idx), line_style));
            value_lines.push(Line::styled(format!("{:0>16b}", self.computer.registers[idx]), line_style));
        }
    
        let key_paragraph = Paragraph::new(key_lines)
            .alignment(Alignment::Left);
        let value_paragraph = Paragraph::new(value_lines)
            .alignment(Alignment::Right);
    
        frame.render_widget(key_paragraph, inner_layout[0]);
        frame.render_widget(value_paragraph, inner_layout[1]);
        frame.render_widget(block, area);
    }
    
    fn render_pm<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" PM : 010 ");
    
        let inner_area = block.inner(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(16),
            ])
            .split(inner_area);
    
        let mut min_idx = self.pm_offset as usize;
        let mut max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, 2usize.pow(8));

        if self.pm_idx < min_idx {
            let diff = min_idx - self.pm_idx;
            self.pm_offset -= diff;
            min_idx -= diff;
            max_idx -= diff;
        } else if self.pm_idx > max_idx - 1 {
            let diff = self.pm_idx - (max_idx - 1);
            self.pm_offset += diff;
            min_idx += diff;
            max_idx += diff;
        }
    
        let mut key_lines = Vec::new();
        let mut value_lines = Vec::new();
    
        for idx in min_idx..max_idx {
            let line_style = if self.selected_comp == Component::PM && idx == self.pm_idx {
                Style::new().fg(Color::Black).bg(Color::White)
            } else {
                Style::new()
            };

            key_lines.push(Line::styled(format!("0x{:0>2x}", idx), line_style));
            value_lines.push(Line::styled(format!("{:0>16b}", self.computer.program_memory[idx]), line_style));
        }
    
        let key_paragraph = Paragraph::new(key_lines)
            .alignment(Alignment::Left);
        let value_paragraph = Paragraph::new(value_lines)
            .alignment(Alignment::Right);
    
        frame.render_widget(key_paragraph, inner_layout[0]);
        frame.render_widget(value_paragraph, inner_layout[1]);
        frame.render_widget(block, area);
    }
    
    fn render_um<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" µM : 111 ");
    
        let inner_area = block.inner(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(25),
            ])
            .split(inner_area);
    
        let mut min_idx = self.um_offset as usize;
        let mut max_idx = min_idx + inner_area.height as usize;
        max_idx = max_idx.clamp(min_idx, 2usize.pow(7));

        if self.um_idx < min_idx {
            let diff = min_idx - self.um_idx;
            self.um_offset -= diff;
            min_idx -= diff;
            max_idx -= diff;
        } else if self.um_idx > max_idx - 1 {
            let diff = self.um_idx - (max_idx - 1);
            self.um_offset += diff;
            min_idx += diff;
            max_idx += diff;
        }
    
        let mut key_lines = Vec::new();
        let mut value_lines = Vec::new();
    
        for idx in min_idx..max_idx {
            let line_style = if self.selected_comp == Component::UM && idx == self.um_idx {
                Style::new().fg(Color::Black).bg(Color::White)
            } else {
                Style::new()
            };

            key_lines.push(Line::styled(format!("0x{:0>2x}", idx),line_style));
            value_lines.push(Line::styled(format!("{:0>25b}", self.computer.micro_memory[idx]), line_style));
        }
    
        let key_paragraph = Paragraph::new(key_lines)
            .alignment(Alignment::Left);
        let value_paragraph = Paragraph::new(value_lines)
            .alignment(Alignment::Right);
    
        frame.render_widget(key_paragraph, inner_layout[0]);
        frame.render_widget(value_paragraph, inner_layout[1]);
        frame.render_widget(block, area);
    }
}