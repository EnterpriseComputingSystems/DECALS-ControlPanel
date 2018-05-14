

mod basic_controls_panel;
mod vertical_menu;
mod container;
mod console;

use conrod;
use conrod::{Ui, UiCell};


use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;


use std::sync::{Arc};


use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};


use self::basic_controls_panel::BasicControlsPanel;
use self::vertical_menu::VerticalMenu;
use self::container::Container;
use self::console::Console;




const MARGIN: Scalar = 5.0;
const PADDING: Scalar = 10.0;


widget_ids! {
    pub struct InterfaceRootIDs {
        canvas,


    }
}

pub struct InterfaceState {
    root_ids: InterfaceRootIDs,
    bcp_state: BasicControlsPanel,
    vm_state: VerticalMenu,
    top_container: Container,
    bottom_container: Container,
    console: Console,
    alert_status: Alert,
    network: Arc<Network>
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, ui: &mut Ui, net: Arc<Network>)-> InterfaceState {
        InterfaceState{root_ids: InterfaceRootIDs::new(ui.widget_id_generator()),
            alert_status: Alert::Normal,
            bcp_state: BasicControlsPanel::new(logo, ui.widget_id_generator()),
            vm_state: VerticalMenu::new(ui, 8),
            bottom_container: Container::new(ui, 2, true, false),
            top_container: Container::new(ui, 2, false, true),
            console: Console::new(ui.widget_id_generator()),
            network: net}
    }

    pub fn set_alert_state(&mut self, alstate: Alert) {
        self.alert_status = alstate;
        Network::change_data_value(&self.network, alert::ALERT_KEY.to_string(), alert::get_alert_text(alstate).to_string());
    }
}

pub fn build_interface(ui: &mut UiCell, interface: &mut InterfaceState) {


    interface.alert_status = alert::get_alert_from_text(interface.network.get_data_value(&alert::ALERT_KEY.to_string()));

    widget::Canvas::new().pad(PADDING).set(interface.root_ids.canvas, ui);

    basic_controls_panel::build(ui, interface);

    interface.vm_state.build(ui, interface.alert_status,
        conrod::widget::Canvas::new().parent(interface.root_ids.canvas)
            .w(200.0)
            .kid_area_h_of(interface.root_ids.canvas)
            .right_from(interface.bcp_state.ids.canvas, MARGIN));

    // Containers
    let mut window = ui.window_dim();
    window = [window[0] - 2.0 * PADDING, window[1] - 2.0 * PADDING];

    let width = window[0] - 302.0 - 200.0 - 2.0 * MARGIN;

    let top_height = 2.0 * window[1] / 3.0;
    let bottom_height = window[1] - top_height - MARGIN;

    let bottom_child_canvas = interface.bottom_container.build(ui, interface.alert_status,
        conrod::widget::Canvas::new().parent(interface.root_ids.canvas)
            .wh([width, bottom_height])
            .bottom_right_of(interface.root_ids.canvas));

    let top_child_canvas = interface.top_container.build(ui, interface.alert_status,
        conrod::widget::Canvas::new().parent(interface.root_ids.canvas)
            .wh([width, top_height])
            .top_right_of(interface.root_ids.canvas));


    interface.console.build(ui, bottom_child_canvas);


}
