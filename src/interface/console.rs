

use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use conrod::{Positionable, Widget, UiCell};
use conrod::widget::Canvas;
use conrod::widget::id::Generator;
use conrod::widget::primitive::text::Text;

use log;
use log::{Record, Level, Metadata};

const MAX_LINES: usize = 12;


widget_ids! {
    pub struct ConsoleIDs {
        canvas,
        text

    }
}

pub struct Console {
    ids: ConsoleIDs,
    txt: Arc<Mutex<LinkedList<String>>>

}

impl Console {
    pub fn new(id_gen: Generator)-> Console {
        Console{ids: ConsoleIDs::new(id_gen),
                txt: Arc::new(Mutex::new(LinkedList::new()))}
    }



    pub fn build(&self, ui: &mut UiCell, base_canvas: Canvas) {


        base_canvas.set(self.ids.canvas, ui);

        let mut txt_out = String::new();
        for t in self.txt.lock().unwrap().iter() {
            txt_out += "\n";
            txt_out += t;
        }

        Text::new(&txt_out).parent(self.ids.canvas)
                .top_left_of(self.ids.canvas)
                .set(self.ids.text, ui);



    }

    // Initialize logger aspect
    pub fn init_logging(&self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(Level::Trace.to_level_filter());
        log::set_boxed_logger(Box::new(ConsoleLogger{txt: self.txt.clone()}))
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
