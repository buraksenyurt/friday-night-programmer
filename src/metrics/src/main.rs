use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Constraint;
use ratatui::prelude::Layout;
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Borders, Chart, Dataset, GraphType};
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::Duration;
use sysinfo::System;

fn main() -> color_eyre::Result<()> {
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
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            system: System::new_all(),
            cpu: Vec::new(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            self.system.refresh_cpu_usage();

            terminal.draw(|frame| {
                self.cpu
                    .push((frame.count() as f64, self.system.global_cpu_usage() as f64));
                self.render(frame)
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let data_set = vec![Dataset::default()
            .name("CPU Usages")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().blue())
            .data(&self.cpu)];

        let chart = Chart::new(data_set)
            .block(Block::bordered().title("CPU Usages"))
            .x_axis(Axis::default().bounds([0.0, self.cpu.len() as f64]))
            .y_axis(Axis::default().bounds([0.0, 100.0]));

        let [top, bottom] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Fill(1)]).areas(frame.area());
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(bottom);

        frame.render_widget(chart, top);
        frame.render_widget(Block::bordered(), bottom);
        frame.render_widget(Block::bordered(), left);
        frame.render_widget(Block::bordered(), right);
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();

        self.draw(frame);

        // let text = "Hello, Ratatui!\n\n\
        //     Created using https://github.com/ratatui/templates\n\
        //     Press `Esc`, `Ctrl-C` or `q` to stop running.";
        // frame.render_widget(
        //     Paragraph::new(text)
        //         .block(Block::bordered().title(title))
        //         .centered(),
        //     frame.area(),
        // )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(30))? {
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
