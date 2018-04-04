

extern crate rand;

use DECALS_base::support::alert::Alert;

use conrod;

widget_ids! {
    pub struct BasicControlsPanelIds {
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
}

impl BasicControlsPanel {
    pub fn new(logo: conrod::image::Id)-> BasicControlsPanel {
        BasicControlsPanel{alert_status: Alert::Normal, logo}
    }
}
