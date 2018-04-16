

mod basic_controls_panel;

use conrod;
use conrod::UiCell;
use conrod::widget::id::Generator;


use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;


use std::sync::{Arc};


use self::basic_controls_panel::BasicControlsPanel;



pub struct InterfaceState {
    bcp_state: BasicControlsPanel,
    alert_status: Alert,
    network: Arc<Network>
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, idgen: Generator, net: Arc<Network>)-> InterfaceState {
        InterfaceState{alert_status: Alert::Normal, bcp_state: BasicControlsPanel::new(logo, idgen), network: net}
    }

    pub fn set_alert_state(&mut self, alstate: Alert) {
        self.alert_status = alstate;
        Network::change_data_value(&self.network, alert::ALERT_KEY.to_string(), alert::get_alert_text(alstate).to_string());
    }
}

pub fn build_interface(ui: &mut UiCell, interface: &mut InterfaceState) {
    interface.alert_status = alert::get_alert_from_text(interface.network.get_data_value(&alert::ALERT_KEY.to_string()));
    basic_controls_panel::build(ui, interface);
}
