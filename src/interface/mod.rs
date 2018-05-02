

mod basic_controls_panel;
mod vertical_menu;

use conrod;
use conrod::{Ui, UiCell};


use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;


use std::sync::{Arc};


use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};


use self::basic_controls_panel::BasicControlsPanel;
use self::vertical_menu::VerticalMenu;


widget_ids! {
    pub struct InterfaceRootIDs {
        canvas,
    }
}

pub struct InterfaceState {
    root_ids: InterfaceRootIDs,
    bcp_state: BasicControlsPanel,
    vm_state: VerticalMenu,
    alert_status: Alert,
    network: Arc<Network>
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, ui: &mut Ui, net: Arc<Network>)-> InterfaceState {
        InterfaceState{root_ids: InterfaceRootIDs::new(ui.widget_id_generator()),
            alert_status: Alert::Normal,
            bcp_state: BasicControlsPanel::new(logo, ui.widget_id_generator()),
            vm_state: VerticalMenu::new(ui.widget_id_generator()),
            network: net}
    }

    pub fn set_alert_state(&mut self, alstate: Alert) {
        self.alert_status = alstate;
        Network::change_data_value(&self.network, alert::ALERT_KEY.to_string(), alert::get_alert_text(alstate).to_string());
    }
}

pub fn build_interface(ui: &mut UiCell, interface: &mut InterfaceState) {
    interface.alert_status = alert::get_alert_from_text(interface.network.get_data_value(&alert::ALERT_KEY.to_string()));

    widget::Canvas::new().pad(10.0).set(interface.root_ids.canvas, ui);

    basic_controls_panel::build(ui, interface);
    vertical_menu::build(ui, interface);
}
