

extern crate rand;

use std::sync::Arc;

use DECALS_base::support::alert::Alert;

use conrod;
use conrod::widget::id::Generator;

use super::InterfaceState;

use super::super::color::{ColorScheme, IteratedScheme};
use super::super::color::colors;
use super::super::color::colors::{Color, Pallette};

widget_ids! {
    pub struct VerticalMenuIDs {
        canvas,

        btn1,
        btn2,
        btn3,
        btn4,
        btn5,
        btn6,
        btn7,
        btn8

    }
}

pub struct VerticalMenu {
    ids: VerticalMenuIDs,
}

impl VerticalMenu {
    pub fn new(idgen: Generator)-> VerticalMenu {
        VerticalMenu{ids: VerticalMenuIDs::new(idgen)}
    }


}

pub fn build(ui: &mut conrod::UiCell, state: &mut InterfaceState) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 30.0;
    const BTN_GAP: conrod::Scalar = 2.0;


    widget::Canvas::new().pad(MARGIN)
        .color(colors::SPACEBLUE)
        .w(200.0)
        .kid_area_h_of(state.root_ids.canvas)
        .align_middle_x_of(state.root_ids.canvas)
        .right(0.0)
        .set(state.vm_state.ids.canvas, ui);



    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Alert buttons

    let mut alert_scheme: Arc<ColorScheme> = Arc::new(match state.alert_status {
        Alert::Normal=> IteratedScheme::new(colors::NO_ALERT.to_vec()),
        Alert::Yellow=> IteratedScheme::new(colors::YELLOW_ALERT.to_vec()),
        Alert::Blue=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Black=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Red=> IteratedScheme::new(colors::RED_ALERT.to_vec())

    });

    for _press in widget::Button::new()
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .label("test1")
        .right_justify_label()
        .mid_top_of(state.vm_state.ids.canvas)
        .w_h(150.0, 70.0)
        .set(state.vm_state.ids.btn1, ui)
        {

        }


}
