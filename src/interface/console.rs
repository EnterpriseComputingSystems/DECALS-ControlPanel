

use std::collections::LinkedList;

use super::super::color::{ColorScheme, IteratedScheme};
use super::super::color::colors;

use conrod;
use conrod::{Colorable, Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;
use conrod::widget::id::Generator;
use conrod::widget::primitive::text::Text;


widget_ids! {
    pub struct ConsoleIDs {
        canvas,
        text

    }
}

pub struct Console {
    ids: ConsoleIDs,
    txt: LinkedList<String>
}

impl Console {
    pub fn new(id_gen: Generator)-> Console {
        Console{ids: ConsoleIDs::new(id_gen),
                txt: LinkedList::new()}
    }



    pub fn build(&self, ui: &mut UiCell, base_canvas: Canvas) {


        base_canvas.color(conrod::color::TRANSPARENT);
        base_canvas.set(self.ids.canvas, ui);

        base_canvas.set(self.ids.canvas, ui);

        let mut txt_out = String::new();
        for t in self.txt.iter() {
            txt_out += t;
        }

        Text::new(&txt_out).parent(self.ids.canvas)
                .top_left_of(self.ids.canvas)
                .set(self.ids.text, ui);



    }

}
