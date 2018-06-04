mod components;
mod panels;
mod widgets;
mod color;



use DECALS_base::support::alert;
use DECALS_base::support::alert::Alert;
use DECALS_base::Network;
use DECALS_base::event::Event;
use DECALS_base::data::{DataReference};



use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc};


use conrod;
use conrod::{Ui, UiCell, Colorable, Positionable, Sizeable, Widget, Scalar};
use conrod::widget::Canvas;


use self::color::ColorScheme;
use self::components::vertical_menu::VerticalMenu;
use self::panels::Panel;
use self::panels::full_image::FullImage;
use self::panels::basic_controls_panel::BasicControlsPanel;
use self::panels::console::Console;
use self::panels::settings_panel::SettingsPanel;

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

struct AvailablePanels {
    large_image: Box<FullImage>,
    settings_panel: Box<SettingsPanel>
}

struct InterfaceResources {
    logo: conrod::image::Id,
    ship_image_1: conrod::image::Id
}

pub struct InterfaceState {
    root_ids: InterfaceRootIDs,
    bcp_state: BasicControlsPanel,
    vm_state: VerticalMenu,
    vm_cs: ColorScheme,
    console: Console,
    resources: InterfaceResources,
    alert_status: DataReference,
    network: Arc<Network>,
    num_devices: usize,
    panel_num: Arc<AtomicUsize>,
    panels: AvailablePanels,
}

impl InterfaceState {
    pub fn new(logo: conrod::image::Id, full_image: conrod::image::Id, ui: &mut Ui, network: Arc<Network>)-> InterfaceState {

        let panel_num = Arc::new(AtomicUsize::new(0));


        // Vertical menu data
        let pntmp = panel_num.clone();
        let vm_btn_handler = move |btn: usize, _: &mut UiCell| {
            pntmp.store(btn, Ordering::Relaxed);
        };

        let vm_labels = VM_LABELS.to_vec().iter().map(|s| s.to_string()).collect();


        let resources = InterfaceResources{logo, ship_image_1: full_image};

        let dm = network.get_data_manager();
        let panels = AvailablePanels{large_image: Box::new(FullImage::new(ui, &dm, false, true, resources.ship_image_1)),
                                     settings_panel: Box::new(SettingsPanel::new(ui, &dm, false, true, &network.get_common_settings()))};

        let interface = InterfaceState{root_ids: InterfaceRootIDs::new(ui.widget_id_generator()),
            alert_status: dm.get_reference(&alert::ALERT_KEY.to_string()),
            bcp_state: BasicControlsPanel::new(logo, ui.widget_id_generator(), network.clone(), &dm),
            vm_state: VerticalMenu::new(ui, VM_NUM_BTNS, vm_labels, Box::new(vm_btn_handler)),
            vm_cs: color::get_suggested_colorscheme(Alert::Normal),
            console: Console::new(ui, &dm, true, false),
            panels,
            panel_num,
            resources,
            network,
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


    // Panel
    let panel_canvas = Canvas::new().parent(interface.root_ids.canvas)
    .wh([width, top_height])
    .top_right_of(interface.root_ids.canvas);

    match interface.panel_num.load(Ordering::Relaxed) {
        7=> interface.panels.settings_panel.build(ui, panel_canvas),
        _=> interface.panels.large_image.build(ui, panel_canvas),
    };

    // Console
    interface.console.build(ui, Canvas::new().parent(interface.root_ids.canvas)
        .wh([width, bottom_height])
        .bottom_right_of(interface.root_ids.canvas));




}
