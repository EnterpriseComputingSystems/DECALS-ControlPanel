extern crate rand;

use super::super::color::ColorScheme;
use super::super::DECALS_widgets::rounded_button::RoundedButton;

use conrod;
use conrod::{Colorable, Labelable, Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;


widget_ids! {
    pub struct VerticalMenuIDs {
        canvas,

        btns[]

    }
}

pub struct VerticalMenu {
    ids: VerticalMenuIDs,
    num_btns: usize
}

impl VerticalMenu {
    pub fn new(ui: &mut Ui, num_btns: usize)-> VerticalMenu {
        let mut ids = VerticalMenuIDs::new(ui.widget_id_generator());
        ids.btns.resize(num_btns, &mut ui.widget_id_generator());
        VerticalMenu{ids, num_btns}
    }



    pub fn build(&self, ui: &mut UiCell, cs: &mut ColorScheme, base_canvas: Canvas) {
        const BTN_GAP: Scalar = 2.0;

        base_canvas.color(conrod::color::TRANSPARENT);
        base_canvas.set(self.ids.canvas, ui);

        let btn_height = (base_canvas.get_h(ui).unwrap() - BTN_GAP * ((self.num_btns - 1) as f64)) / (self.num_btns as f64);

        for _press in RoundedButton::new()
            .color(cs.get_next_color())
            .mid_top_of(self.ids.canvas)
            .h(btn_height)
            .set(self.ids.btns[0], ui) {

        }

        for i in 1..self.num_btns {
            for _press in RoundedButton::new()
                .color(cs.get_next_color())
                .down(BTN_GAP)
                .h(btn_height)
                .set(self.ids.btns[i], ui) {

            }
        }


    }

}
