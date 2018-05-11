

extern crate rand;

use std::sync::Arc;

use DECALS_base::support::alert::Alert;

use super::super::color::{ColorScheme, IteratedScheme};
use super::super::color::colors;
use super::super::color::colors::{Color, Pallette};
use super::super::DECALS_widgets::rounded_button::RoundedButton;

use conrod::{Colorable, Labelable, Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;
use conrod::widget::id::Generator;


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



    pub fn build(&self, ui: &mut UiCell, alert_status: Alert, base_canvas: Canvas) {
        const BTN_GAP: Scalar = 2.0;

        base_canvas.set(self.ids.canvas, ui);

        let btn_height = (base_canvas.get_h(ui).unwrap() - BTN_GAP * ((self.num_btns - 1) as f64)) / (self.num_btns as f64);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Alert buttons

        let mut alert_scheme: Arc<ColorScheme> = Arc::new(match alert_status {
            Alert::Normal=> IteratedScheme::new(colors::NO_ALERT.to_vec()),
            Alert::Yellow=> IteratedScheme::new(colors::YELLOW_ALERT.to_vec()),
            Alert::Blue=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
            Alert::Black=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
            Alert::Red=> IteratedScheme::new(colors::RED_ALERT.to_vec())

        });

        for _press in RoundedButton::new()
            .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
            .mid_top_of(self.ids.canvas)
            .h(btn_height)
            .set(self.ids.btns[0], ui) {

        }

        for i in 1..self.num_btns {
            for _press in RoundedButton::new()
                .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
                .down(BTN_GAP)
                .h(btn_height)
                .set(self.ids.btns[i], ui) {

            }
        }


    }

}
