use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Constraint;
use ratatui::prelude::{Layout, Line};
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Bar, BarChart, BarGroup, Chart, Dataset, GraphType};
use ratatui::{style::Stylize, widgets::Block, DefaultTerminal, Frame};
use std::time::Duration;
use sysinfo::System;

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
            self.system.refresh_all();

            terminal.draw(|frame| {
                self.cpu
                    .push((frame.count() as f64, self.system.global_cpu_usage() as f64));
                self.used_memory.push(self.system.used_memory());

                self.render(frame)
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
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
            .block(Block::new().title(format!("Total Memory {} Gb", total_memory)))
            .bar_width(8)
            .bar_gap(2)
            .max(total_memory as u64);

        let [top, bottom] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Fill(1)]).areas(frame.area());
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(bottom);

        frame.render_widget(cpu_chart, top);
        frame.render_widget(Block::bordered(), bottom);
        frame.render_widget(memory_chart, left);
        frame.render_widget(Block::bordered(), right);
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
