use crate::scanner::PortInfo;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

pub struct App {
    pub ports: Vec<PortInfo>,
    pub table_state: TableState,
    pub interval: u64,
    pub last_update: std::time::Instant,
}

impl App {
    pub fn new(interval: u64) -> Self {
        App {
            ports: Vec::new(),
            table_state: TableState::default(),
            interval,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn update_ports(&mut self, ports: Vec<PortInfo>) {
        self.ports = ports;
        self.last_update = std::time::Instant::now();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_table(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    pub fn render_header(&mut self, frame: &mut Frame, area: Rect) {
        let elapsed = self.last_update.elapsed().as_secs();
        let title = format!(
            "portwatch | {} ports | last update: {}s ago | interval: {}s",
            self.ports.len(),
            elapsed,
            self.interval
        );
        let block = Paragraph::new(title)
            .style(Style::default())
            .bg(Color::Rgb(180, 65, 14))
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        frame.render_widget(block, area);
    }

    pub fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_chells = ["PORT", "PROTO", "PID", "PROCESS"].iter().map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        });

        let header = Row::new(header_chells)
            .style(Style::default().bg(Color::DarkGray))
            .height(1);

        let rows = self.ports.iter().map(|p| {
            let cells = vec![
                Cell::from(p.port.to_string()).style(Style::default().fg(Color::Cyan)),
                Cell::from(p.protocol.clone()).style(Style::default().fg(Color::Yellow)),
                Cell::from(p.pid.map(|id| id.to_string()).unwrap_or("-".into()))
                    .style(Style::default().fg(Color::DarkGray)),
                Cell::from(p.process_name.clone().unwrap_or("unknown".into()))
                    .style(Style::default().fg(Color::Green)),
            ];
            Row::new(cells).height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Open Ports "))
        .row_highlight_style(Style::default())
        .bg(Color::Rgb(30, 111, 165))
        .add_modifier(Modifier::BOLD);

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    pub fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        let help = Paragraph::new(" q: quit up/down: navigate r: refresh now ")
            .style(Style::default().bg(Color::DarkGray).fg(Color::Gray));
        frame.render_widget(help, area);
    }
}
