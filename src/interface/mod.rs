

mod basic_controls_panel;
mod vertical_menu;
mod container;
mod console;

use conrod;
use conrod::{Ui, UiCell};


use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;
use DECALS_base::event::Event;

use super::color::ColorScheme;
use super::color::colors;


use std::sync::{Arc};


use conrod::{Colorable, Positionable, Sizeable, Widget, Scalar};
use conrod::widget::Canvas;


use self::basic_controls_panel::BasicControlsPanel;
use self::vertical_menu::VerticalMenu;
use self::container::Container;
use self::console::Console;

const MARGIN: Scalar = 5.0;
const PADDING: Scalar = 10.0;

const VM_WIDTH: Scalar = 200.0;
const VM_NUM_BTNS: usize = 8;
const VM_LABELS: [&str; 8] = ["", "", "", "", "", "", "", "Settings"];


widget_ids! {
    pub struct InterfaceRootIDs {
        canvas,


    }
}

pub struct InterfaceState {
    root_ids: InterfaceRootIDs,
    bcp_state: BasicControlsPanel,
    vm_state: VerticalMenu,
    vm_cs: ColorScheme,
    top_container: Container,
    bottom_container: Container,
    console: Console,
    alert_status: Alert,
    network: Arc<Network>,
    num_devices: usize,
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, ui: &mut Ui, net: Arc<Network>)-> InterfaceState {

        let vm_btn_handler = |btn: usize| {
            warn!("{}", btn);
        };

        let vm_labels = VM_LABELS.to_vec().iter().map(|s| s.to_string()).collect();

        let interface = InterfaceState{root_ids: InterfaceRootIDs::new(ui.widget_id_generator()),
            alert_status: Alert::Normal,
            bcp_state: BasicControlsPanel::new(logo, ui.widget_id_generator()),
            vm_state: VerticalMenu::new(ui, VM_NUM_BTNS, vm_labels, Box::new(vm_btn_handler)),
            vm_cs: get_colorscheme(Alert::Normal),
            bottom_container: Container::new(ui, 2, true, false),
            top_container: Container::new(ui, 2, false, true),
            console: Console::new(ui.widget_id_generator()),
            network: net,
            num_devices: 1};


        interface.console.init_logging().unwrap();
        return interface;
    }
}


/// Construct the DECALS interface
/// Responsive to changes in window size
pub fn build_interface(ui: &mut UiCell, interface: &mut InterfaceState) {

    {
        let rec = interface.network.event_receiver.lock().unwrap();
        loop {
            match rec.try_recv() {
                Ok(Event::DataChange(dp))=>match dp.key.as_ref() {
                    alert::ALERT_KEY=>{
                        interface.alert_status = alert::get_alert_from_text(dp.value);
                        interface.vm_cs = get_colorscheme(interface.alert_status);
                    },
                    _=>{}
                },
                Ok(Event::UnitDiscovered(_))=>interface.num_devices = interface.network.get_num_devices(),
                Err(_)=>break
            }
        }
    }

    // ROot canvas
    Canvas::new()
        .pad(PADDING)
        .color(conrod::color::BLACK)
        .set(interface.root_ids.canvas, ui);

    // Basic Controls panel
    interface.bcp_state.build(ui, interface.alert_status, &interface.network, Canvas::new()
        .kid_area_h_of(interface.root_ids.canvas)
        .top_left_of(interface.root_ids.canvas));

    // Vertical Menu
    interface.vm_cs.reset_to_start();

    interface.vm_state.build(ui, &mut interface.vm_cs,
        Canvas::new().parent(interface.root_ids.canvas)
            .w(VM_WIDTH)
            .kid_area_h_of(interface.root_ids.canvas)
            .right_from(interface.bcp_state.ids.canvas, MARGIN));

    // Containers
    let mut window = ui.window_dim();
    window = [window[0] - 2.0 * PADDING, window[1] - 2.0 * PADDING];

    let width = window[0] - 302.0 - VM_WIDTH - 2.0 * MARGIN;

    let top_height = 2.0 * window[1] / 3.0;
    let bottom_height = window[1] - top_height - MARGIN;

    let bottom_child_canvas = interface.bottom_container.build(ui, interface.alert_status,
        Canvas::new().parent(interface.root_ids.canvas)
            .wh([width, bottom_height])
            .bottom_right_of(interface.root_ids.canvas));

    let top_child_canvas = interface.top_container.build(ui, interface.alert_status,
        Canvas::new().parent(interface.root_ids.canvas)
            .wh([width, top_height])
            .top_right_of(interface.root_ids.canvas));


    // Console
    interface.console.build(ui, bottom_child_canvas);

}

pub fn get_colorscheme(al: Alert) ->ColorScheme {
    match al {
        Alert::Normal=> ColorScheme::new(colors::NO_ALERT.to_vec()),
        Alert::Yellow=> ColorScheme::new(colors::YELLOW_ALERT.to_vec()),
        Alert::Blue=> ColorScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Black=> ColorScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Red=> ColorScheme::new(colors::RED_ALERT.to_vec())
    }

}
