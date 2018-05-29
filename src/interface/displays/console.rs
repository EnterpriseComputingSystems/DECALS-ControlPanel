

use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use conrod::{Positionable, Widget, UiCell, Ui};
use conrod::widget::Canvas;
use conrod::widget::primitive::text::Text;

use log;
use log::{Record, Level, Metadata};

use DECALS_base::data::DataManager;

use super::Display;
use super::super::components::container::Container;

const MAX_LINES: usize = 10;

const CONT_BTNS: usize = 2;
const CONT_BTN_LABELS: [&str; 2] = ["Up", "Down"];


widget_ids! {
    pub struct ConsoleIDs {
        canvas,
        text

    }
}

pub struct Console {
    ids: ConsoleIDs,
    container: Container,
    txt: Arc<Mutex<LinkedList<String>>>

}

impl Console {
    pub fn new(ui: &mut Ui, dm: &DataManager, top_border: bool, bottom_border: bool)-> Console {

        let labels = CONT_BTN_LABELS.to_vec().iter().map(|s| s.to_string()).collect();

        let ct_btn_handler = |btn: usize| {
            match btn {
                _=>()
            }
        };

        Console{ids: ConsoleIDs::new(ui.widget_id_generator()),
                container: Container::new(ui, CONT_BTNS, top_border, bottom_border, dm, labels, Box::new(ct_btn_handler)),
                txt: Arc::new(Mutex::new(LinkedList::new()))}
    }

    // Initialize logger aspect
    pub fn init_logging(&self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(Level::Trace.to_level_filter());
        log::set_boxed_logger(Box::new(ConsoleLogger{txt: self.txt.clone()}))
    }

}

impl Display for Console {

    fn build(&mut self, ui: &mut UiCell, base_canvas: Canvas) {

        let sub_canvas = self.container.build(ui, base_canvas);
        sub_canvas.set(self.ids.canvas, ui);

        let mut txt_out = String::new();
        for t in self.txt.lock().unwrap().iter() {
            txt_out += "\n";
            txt_out += t;
        }

        Text::new(&txt_out).parent(self.ids.canvas)
                .top_left_of(self.ids.canvas)
                .set(self.ids.text, ui);
    }
}

// Logs information to the consoloe and to stdout
struct ConsoleLogger {
    txt: Arc<Mutex<LinkedList<String>>>
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut lst = self.txt.lock().unwrap();

            if lst.len() == MAX_LINES {
                lst.pop_front();
            }

            let out = format!("{} - {}", record.level(), record.args());
            lst.push_back(out.clone());

            println!("{}", out);


        }
    }

    fn flush(&self) {}
}
