extern crate rand;

use super::super::color::ColorScheme;
use super::super::DECALS_widgets::rounded_button::RoundedButton;

use conrod::{Colorable, Labelable, Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;


const BTN_GAP: Scalar = 2.0;

widget_ids! {
    pub struct VerticalMenuIDs {
        canvas,
        btns[]
    }
}

// A vertical column of buttons
pub struct VerticalMenu {
    ids: VerticalMenuIDs,
    num_btns: usize,
    labels: Vec<String>,
    click_handler: Box<Fn(usize)>
}

impl VerticalMenu {
    pub fn new(ui: &mut Ui, num_btns: usize, labels: Vec<String>, click_handler: Box<Fn(usize)>)-> VerticalMenu {

        assert!(num_btns >= 1);
        assert!(labels.len() == num_btns);

        let mut ids = VerticalMenuIDs::new(ui.widget_id_generator());
        ids.btns.resize(num_btns, &mut ui.widget_id_generator());
        VerticalMenu{ids, num_btns, labels, click_handler}
    }


    // Build the menu on the given canvas
    pub fn build(&self, ui: &mut UiCell, cs: &mut ColorScheme, base_canvas: Canvas) {

        base_canvas.set(self.ids.canvas, ui);

        let btn_height = (base_canvas.get_h(ui).unwrap() - BTN_GAP * ((self.num_btns - 1) as f64)) / (self.num_btns as f64);


        // Create buttons
        for i in 0..self.num_btns {

            let mut next_btn = RoundedButton::new().color(cs.get_next_color())
                .h(btn_height);

            // Set the label if neccessary
            if self.labels[i] != "" {
                next_btn = next_btn.label(&self.labels[i]);
            }

            // If the first button, set it at the top.
            if i == 0 {
                next_btn = next_btn.mid_top_of(self.ids.canvas);
            } else {
                next_btn = next_btn.down(BTN_GAP)
            }

            // Handle button presses
            for _press in next_btn.set(self.ids.btns[i], ui) {
                (self.click_handler)(i);
            }
        }


    }

}
