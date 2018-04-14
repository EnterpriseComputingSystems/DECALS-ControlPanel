

mod basic_controls_panel;

use conrod;
use conrod::UiCell;
use conrod::widget::id::Generator;

use self::basic_controls_panel::BasicControlsPanel;


pub struct InterfaceState {
    bcp_state: BasicControlsPanel,
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, idgen: Generator)-> InterfaceState {
        InterfaceState{bcp_state: BasicControlsPanel::new(logo, idgen)}
    }
}

pub fn build_interface(ui: &mut conrod::UiCell, interface: &InterfaceState) {
    BasicControlsPanel::build(ui, &interface.bcp_state);
}
