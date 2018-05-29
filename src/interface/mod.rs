mod components;
mod displays;
mod widgets;
mod color;



use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;
use DECALS_base::event::Event;
use DECALS_base::data::{DataReference};




use std::sync::{Arc};


use conrod;
use conrod::{Ui, UiCell, Colorable, Positionable, Sizeable, Widget, Scalar};
use conrod::widget::Canvas;


use self::color::ColorScheme;
use self::components::vertical_menu::VerticalMenu;
use self::displays::Display;
use self::displays::full_image::FullImage;
use self::displays::basic_controls_panel::BasicControlsPanel;
use self::displays::console::Console;

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
    console: Console,
    fullimg: FullImage,
    alert_status: DataReference,
    network: Arc<Network>,
    num_devices: usize,
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, full_image: conrod::image::Id, ui: &mut Ui, net: Arc<Network>)-> InterfaceState {

        let dm = net.get_data_manager();

        let vm_btn_handler = |btn: usize| {
            match btn {
                _=>()
            }
        };

        let vm_labels = VM_LABELS.to_vec().iter().map(|s| s.to_string()).collect();

        let interface = InterfaceState{root_ids: InterfaceRootIDs::new(ui.widget_id_generator()),
            alert_status: dm.get_reference(&alert::ALERT_KEY.to_string()),
            bcp_state: BasicControlsPanel::new(logo, ui.widget_id_generator(), net.clone(), &dm),
            vm_state: VerticalMenu::new(ui, VM_NUM_BTNS, vm_labels, Box::new(vm_btn_handler)),
            vm_cs: color::get_suggested_colorscheme(Alert::Normal),
            console: Console::new(ui, &dm, true, false),
            fullimg: FullImage::new(ui, &dm, false, true, full_image),
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
                Ok(Event::UnitDiscovered(_))=>interface.num_devices = interface.network.get_num_devices(),
                Err(_)=>break,
                _=>{}
            }
        }
    }


    // ROot canvas
    Canvas::new()
        .pad(PADDING)
        .color(conrod::color::BLACK)
        .set(interface.root_ids.canvas, ui);

    // Basic Controls panel
    interface.bcp_state.build(ui, Canvas::new()
        .kid_area_h_of(interface.root_ids.canvas)
        .top_left_of(interface.root_ids.canvas));

    // Vertical Menu
    if interface.alert_status.test_changed() {
        interface.vm_cs = color::get_suggested_colorscheme(alert::get_alert_from_text(interface.alert_status.get_value()));
    } else {
        interface.vm_cs.reset_to_start();
    }

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

    interface.fullimg.build(ui,
        Canvas::new().parent(interface.root_ids.canvas)
            .wh([width, top_height])
            .top_right_of(interface.root_ids.canvas));


    // Console
    interface.console.build(ui, Canvas::new().parent(interface.root_ids.canvas)
        .wh([width, bottom_height])
        .bottom_right_of(interface.root_ids.canvas));

}
