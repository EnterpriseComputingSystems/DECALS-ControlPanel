

extern crate rand;

use std::sync::Arc;

use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;
use DECALS_base::data::{DataManager, DataReference};

use conrod;
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use conrod::widget::{Canvas};
use conrod::widget::id::Generator;
use conrod::widget::primitive::text::Text;

use super::Panel;
use super::super::widgets::rounded_button::RoundedButton;
use super::super::color;

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
        nav_8,

        status_text

    }
}

pub struct BasicControlsPanel {
    logo: conrod::image::Id,
    pub ids: BasicControlsPanelIds,
    alert_status: DataReference,
    network: Arc<Network>
}

impl BasicControlsPanel {
    pub fn new(logo: conrod::image::Id, idgen: Generator, network: Arc<Network>, dm: &DataManager)-> BasicControlsPanel {
        BasicControlsPanel{logo,
            ids: BasicControlsPanelIds::new(idgen),
            alert_status: dm.get_reference(&alert::ALERT_KEY.to_string()),
            network}
    }
}

impl Panel for BasicControlsPanel {
    fn build(&mut self, ui: &mut conrod::UiCell, base_canvas: Canvas) {

        const MARGIN: conrod::Scalar = 15.0;
        const BTN_GAP: conrod::Scalar = 2.0;
        const BTN_RADIUS: conrod::Scalar = 25.0;

        const LOGO_SIZE: conrod::Scalar = 200.0;


        base_canvas.w(302.0)
            .set(self.ids.canvas, ui);

        widget::Image::new(self.logo)
        .w_h(LOGO_SIZE, LOGO_SIZE)
        .mid_top_of(self.ids.canvas)
        .set(self.ids.logo, ui);



        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Alert buttons

        let mut cscheme = color::get_suggested_colorscheme(alert::get_alert_from_text(self.alert_status.get_value()));

        widget::Canvas::new()
        .down(MARGIN)
        .align_middle_x_of(self.ids.canvas)
        .kid_area_w_of(self.ids.canvas)
        .h(295.0)
        .flow_down(&[
            (self.ids.alert_row_1, widget::Canvas::new()),
            (self.ids.alert_row_2, widget::Canvas::new()),
            (self.ids.alert_row_3, widget::Canvas::new()),
            (self.ids.alert_row_4, widget::Canvas::new())
            ])
            .set(self.ids.alert_canvas, ui);

        for _press in RoundedButton::rounded_left(BTN_RADIUS)
            .color(cscheme.get_next_color())
            .mid_left_of(self.ids.alert_row_1)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_1_1, ui)
            {

            }

        for _press in RoundedButton::new()
            .color(cscheme.get_next_color())
            .label(&alert::get_alert_text(Alert::Normal))
            .right(BTN_GAP)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_2_1, ui)
            {
                self.alert_status.update_data(alert::get_alert_text(Alert::Normal).to_string());
            }

        for _press in RoundedButton::rounded_left(BTN_RADIUS)
            .color(cscheme.get_next_color())
            .mid_left_of(self.ids.alert_row_2)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_1_2, ui)
            {

            }

        for _press in RoundedButton::new()
            .color(cscheme.get_next_color())
            .label(&alert::get_alert_text(Alert::Blue))
            .right(BTN_GAP)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_2_2, ui)
            {
                self.alert_status.update_data(alert::get_alert_text(Alert::Blue).to_string());
            }

        for _press in RoundedButton::rounded_left(BTN_RADIUS)
            .color(cscheme.get_next_color())
            .mid_left_of(self.ids.alert_row_3)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_1_3, ui)
            {

            }

        for _press in RoundedButton::new()
            .color(cscheme.get_next_color())
            .label(&alert::get_alert_text(Alert::Yellow))
            .right(BTN_GAP)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_2_3, ui)
            {
                self.alert_status.update_data(alert::get_alert_text(Alert::Yellow).to_string());
            }

        for _press in RoundedButton::rounded_left(BTN_RADIUS)
            .color(cscheme.get_next_color())
            .mid_left_of(self.ids.alert_row_4)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_1_4, ui)
            {

            }

        for _press in RoundedButton::new()
            .color(cscheme.get_next_color())
            .label(&alert::get_alert_text(Alert::Red))
            .right(BTN_GAP)
            .w_h(150.0, 70.0)
            .set(self.ids.alert_2_4, ui)
            {
                self.alert_status.update_data(alert::get_alert_text(Alert::Red).to_string());
            }

        let status = format!("Discovered devices: {}\nDevice id: {}\nDevice IP: {}\nDevice port: {}",
                            self.network.get_num_devices(),
                            self.network.get_id(),
                            self.network.get_ip(),
                            self.network.get_port());

        Text::new(&status).parent(self.ids.canvas)
            .down_from(self.ids.alert_canvas, MARGIN)
            .set(self.ids.status_text, ui);

    }
}
