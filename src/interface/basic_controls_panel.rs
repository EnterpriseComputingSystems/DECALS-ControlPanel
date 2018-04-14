

extern crate rand;

use DECALS_base::support::alert::Alert;

use conrod;
use conrod::UiCell;
use conrod::widget::id::Generator;

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
    alert_status: Alert,
    logo: conrod::image::Id,
    ids: BasicControlsPanelIds
}

impl BasicControlsPanel {
    pub fn new(logo: conrod::image::Id, idgen: Generator)-> BasicControlsPanel {
        BasicControlsPanel{alert_status: Alert::Normal, logo, ids: BasicControlsPanelIds::new(idgen)}
    }


    pub fn build(ui: &mut conrod::UiCell, bcp: &BasicControlsPanel) {
        use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
        use std::iter::once;

        const MARGIN: conrod::Scalar = 30.0;
        const UTIL_GAP: conrod::Scalar = 5.0;

        const LOGO_SIZE: conrod::Scalar = 200.0;


        widget::Canvas::new().pad(MARGIN).set(bcp.ids.canvas, ui);

        widget::Image::new(bcp.logo)
            .w_h(LOGO_SIZE, LOGO_SIZE)
            .mid_top_of(bcp.ids.canvas)
            .set(bcp.ids.logo, ui);

        widget::Canvas::new()
            .down(0.0)
            .align_middle_x_of(bcp.ids.canvas)
            .kid_area_w_of(bcp.ids.canvas)
            .h(360.0)
            .color(conrod::color::TRANSPARENT)
            .pad(MARGIN)
            .flow_down(&[
                (bcp.ids.alert_row_1, widget::Canvas::new()),
                (bcp.ids.alert_row_2, widget::Canvas::new()),
                (bcp.ids.alert_row_3, widget::Canvas::new()),
                (bcp.ids.alert_row_4, widget::Canvas::new())
            ])
            .set(bcp.ids.alert_canvas, ui);




    }
}
