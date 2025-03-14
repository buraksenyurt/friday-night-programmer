use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Constraint;
use ratatui::prelude::{Layout, Line, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::{
    Axis, Bar, BarChart, BarGroup, Borders, Chart, Dataset, GraphType, Row, Table,
};
use ratatui::{style::Stylize, widgets::Block, DefaultTerminal, Frame};
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    system: System,
    cpu: Vec<(f64, f64)>,
    used_memory: Vec<u64>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            system: System::new_all(),
            cpu: Vec::new(),
            used_memory: Vec::new(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                if frame.count() % 5 == 0 {
                    self.system.refresh_processes(ProcessesToUpdate::All, true);

                    self.system.refresh_memory();
                    self.used_memory.push(self.system.used_memory());
                }

                self.system.refresh_cpu_usage();
                self.cpu
                    .push((frame.count() as f64, self.system.global_cpu_usage() as f64));

                self.render(frame);
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [top, bottom] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Fill(1)]).areas(frame.area());
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(bottom);

        frame.render_widget(Block::bordered(), bottom);

        self.render_cpu(frame, &top);
        self.render_memory(frame, &left);
        self.render_processes(frame, &right);
    }

    fn render_cpu(&self, frame: &mut Frame, rect: &Rect) {
        let cpu_data_set = vec![Dataset::default()
            .name("Cpu Usage")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().blue())
            .data(&self.cpu)];

        let cpu_chart = Chart::new(cpu_data_set)
            .block(Block::bordered().title("CPU Usages"))
            .x_axis(Axis::default().bounds([0.0, self.cpu.len() as f64]))
            .y_axis(Axis::default().bounds([0.0, 100.0]));

        frame.render_widget(cpu_chart, *rect);
    }

    fn render_memory(&self, frame: &mut Frame, rect: &Rect) {
        let memory_bars: Vec<Bar> = self
            .used_memory
            .iter()
            .enumerate()
            .map(|(idx, used_memory)| {
                Bar::default()
                    .value(*used_memory)
                    .label(Line::from(format!("{idx:>02}")))
                    .text_value(format!(
                        "{}",
                        (*used_memory as f64 / (1024 * 1024) as f64).round()
                    ))
                    //.style(Style::new().fg(Color::Rgb(255, 255, 0)))
                    .value_style(Style::new().fg(Color::Rgb(255, 255, 255)))
            })
            .collect();

        let total_memory =
            (self.system.total_memory() as f64 / (1024 * 1024 * 1024) as f64).round() as usize;
        let memory_chart = BarChart::default()
            .data(BarGroup::default().bars(&memory_bars))
            .block(
                Block::new()
                    .title(format!("Total Memory {} Gb", total_memory))
                    .borders(Borders::ALL),
            )
            .bar_width(8)
            .bar_gap(2)
            .max(total_memory as u64);

        frame.render_widget(memory_chart, *rect);
    }

    fn render_processes(&self, frame: &mut Frame, rect: &Rect) {
        let mut process_data: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                vec![
                    pid.to_string(),
                    process.name().to_string_lossy().to_string(),
                    process.cpu_usage().to_string(),
                    (process.memory() as f64 / (1024 * 1024) as f64).to_string(),
                ]
            })
            .collect();
        process_data.sort_by(|p1, p2| p2[2].cmp(&p1[2]));
        let rows: Vec<Row> = process_data.into_iter().map(Row::new).collect();
        let process_table = Table::new(
            rows,
            [
                Constraint::Max(10),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .block(Block::default().title("Processes").borders(Borders::ALL))
        .header(
            Row::new(vec!["Process Id", "Name", "CPU", "Memory"])
                .style(Style::default().bold().fg(Color::Blue)),
        )
        .column_spacing(1);

        frame.render_widget(process_table, *rect);
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        self.draw(frame);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
