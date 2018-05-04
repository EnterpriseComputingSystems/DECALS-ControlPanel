

extern crate rand;

use std::sync::Arc;

use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;

use conrod;
use conrod::widget::id::Generator;

use super::InterfaceState;

use super::super::color::{ColorScheme, IteratedScheme};
use super::super::color::colors;
use super::super::color::colors::{Color, Pallette};

widget_ids! {
    pub struct BasicControlsPanelIds {
        canvas,

        logo,

        //Butons
        alert_1_1,
        alert_1_2,
        alert_1_3,
        alert_1_4,
        alert_2_1,
        alert_2_2,
        alert_2_3,
        alert_2_4,

        alert_row_1,
        alert_row_2,
        alert_row_3,
        alert_row_4,

        alert_canvas,

        util_1_1,
        util_1_2,
        util_1_3,
        util_1_4,
        util_2_1,
        util_2_2,
        util_2_3,
        util_2_4,

        nav_1,
        nav_2,
        nav_3,
        nav_4,
        nav_5,
        nav_6,
        nav_7,
        nav_8

    }
}

pub struct BasicControlsPanel {
    logo: conrod::image::Id,
    pub ids: BasicControlsPanelIds,
}

impl BasicControlsPanel {
    pub fn new(logo: conrod::image::Id, idgen: Generator)-> BasicControlsPanel {
        BasicControlsPanel{logo, ids: BasicControlsPanelIds::new(idgen)}
    }


}

pub fn build(ui: &mut conrod::UiCell, state: &mut InterfaceState) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use super::super::DECALS_widgets::rounded_button::RoundedButton;

    const MARGIN: conrod::Scalar = 15.0;
    const BTN_GAP: conrod::Scalar = 2.0;
    const BTN_RADIUS: conrod::Scalar = 25.0;

    const LOGO_SIZE: conrod::Scalar = 200.0;


    widget::Canvas::new()
        .color(conrod::color::TRANSPARENT)
        .w(302.0)
        .kid_area_h_of(state.root_ids.canvas)
        .top_left_of(state.root_ids.canvas)
        .set(state.bcp_state.ids.canvas, ui);

    widget::Image::new(state.bcp_state.logo)
    .w_h(LOGO_SIZE, LOGO_SIZE)
    .mid_top_of(state.bcp_state.ids.canvas)
    .set(state.bcp_state.ids.logo, ui);



    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Alert buttons

    let mut alert_scheme: Arc<ColorScheme> = Arc::new(match state.alert_status {
        Alert::Normal=> IteratedScheme::new(colors::NO_ALERT.to_vec()),
        Alert::Yellow=> IteratedScheme::new(colors::YELLOW_ALERT.to_vec()),
        Alert::Blue=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Black=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Red=> IteratedScheme::new(colors::RED_ALERT.to_vec())

    });

    widget::Canvas::new()
    .down(MARGIN)
    .align_middle_x_of(state.bcp_state.ids.canvas)
    .kid_area_w_of(state.bcp_state.ids.canvas)
    .h(310.0)
    .color(conrod::color::TRANSPARENT)
    .flow_down(&[
        (state.bcp_state.ids.alert_row_1, widget::Canvas::new().color(conrod::color::TRANSPARENT)),
        (state.bcp_state.ids.alert_row_2, widget::Canvas::new().color(conrod::color::TRANSPARENT)),
        (state.bcp_state.ids.alert_row_3, widget::Canvas::new().color(conrod::color::TRANSPARENT)),
        (state.bcp_state.ids.alert_row_4, widget::Canvas::new().color(conrod::color::TRANSPARENT))
        ])
        .set(state.bcp_state.ids.alert_canvas, ui);

    for _press in RoundedButton::rounded_left(BTN_RADIUS)
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .mid_left_of(state.bcp_state.ids.alert_row_1)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_1_1, ui)
        {

        }

    for _press in RoundedButton::new()
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .label(&alert::get_alert_text(Alert::Normal))
        .right(BTN_GAP)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_2_1, ui)
        {
            state.set_alert_state(Alert::Normal);
        }

    for _press in RoundedButton::rounded_left(BTN_RADIUS)
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .mid_left_of(state.bcp_state.ids.alert_row_2)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_1_2, ui)
        {

        }

    for _press in RoundedButton::new()
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .label(&alert::get_alert_text(Alert::Blue))
        .right(BTN_GAP)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_2_2, ui)
        {
            state.set_alert_state(Alert::Blue);
        }

    for _press in RoundedButton::rounded_left(BTN_RADIUS)
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .mid_left_of(state.bcp_state.ids.alert_row_3)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_1_3, ui)
        {

        }

    for _press in RoundedButton::new()
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .label(&alert::get_alert_text(Alert::Yellow))
        .right(BTN_GAP)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_2_3, ui)
        {
            state.set_alert_state(Alert::Yellow);
        }

    for _press in RoundedButton::rounded_left(BTN_RADIUS)
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .mid_left_of(state.bcp_state.ids.alert_row_4)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_1_4, ui)
        {

        }

    for _press in RoundedButton::new()
        .color(Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
        .label(&alert::get_alert_text(Alert::Red))
        .right(BTN_GAP)
        .w_h(150.0, 70.0)
        .set(state.bcp_state.ids.alert_2_4, ui)
        {
            state.set_alert_state(Alert::Red);
        }


}
