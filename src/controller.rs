use std::error::Error;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;
use tokio::task;

use crate::{
    model::{Model, Page},
    screen::canvas::{Canvas, RootViewNode},
};
pub struct Controller {
    canvas: Canvas,
    model: Model,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            canvas: Canvas::new(60, 20),
            model: Model::new(),
        }
    }

    pub async fn init(&mut self) {
        self.render();
    }

    fn render(&mut self) {
        let view_node = self.model.state.home_entry_state.render().to_view_mut();

        let root_view_node = RootViewNode(view_node);

        self.canvas.render_view_node_tree(&root_view_node);

        self.canvas.draw_on_screen();
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        task::spawn(async move {
            while let Ok(event) = event::read() {
                if let event::Event::Key(key_event) = event {
                    if let Err(_) = tx.send(key_event) {
                        break;
                    }
                }
            }
        });

        loop {
            if let Some(key_event) = rx.recv().await {
                match (key_event, self.model.current_page) {
                    (
                        KeyEvent {
                            code: KeyCode::Esc, ..
                        },
                        Page::HomeEntry,
                    ) => {
                        break;
                    }
                    _ => {}
                }
                match key_event {
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code,
                        ..
                    } => {
                        self.control_home_page(code);
                        self.render();
                    }
                    _ => {}
                };
            }
        }

        Ok(())
    }

    fn control_home_page(&mut self, key_code: KeyCode) {
        let home_state = &mut self.model.state.home_entry_state;

        match key_code {
            KeyCode::Up => {
                home_state.to_previous_item();
            }
            KeyCode::Down => {
                home_state.to_next_item();
            }
            _ => {}
        }
    }
}
