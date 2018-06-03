

use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use conrod::{Positionable, Sizeable, Widget, UiCell, Ui};
use conrod::widget::Canvas;
use conrod::widget::primitive::text::Text;

use log;
use log::{Record, Level, Metadata};

use time;

use DECALS_base::data::DataManager;

use super::Display;
use super::super::components::container::Container;

const MAX_LINES: usize = 30;

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

        let ids = ConsoleIDs::new(ui.widget_id_generator());

        let canvasid = ids.canvas;
        let ct_btn_handler = move |btn: usize, ui: &mut UiCell| {
            match btn {
                0=>ui.scroll_widget(canvasid, [0.0, 50.0]),
                1=>ui.scroll_widget(canvasid, [0.0, -50.0]),
                _=>()
            }
        };

        Console{ids,
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
        sub_canvas.scroll_kids_vertically()
            .set(self.ids.canvas, ui);

        let mut txt_out = String::new();
        for t in self.txt.lock().unwrap().iter() {
            txt_out += "\n";
            txt_out += t;
        }

        Text::new(&txt_out).parent(self.ids.canvas)
                .h(500.0)
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
                lst.pop_back();
            }

            let out = format!("{}: {} - {}", time::strftime("%H-%M-%S", &time::now()).unwrap(), record.level(), record.args());
            lst.push_front(out.clone());

            println!("{}", out);


        }
    }

    fn flush(&self) {}
}
