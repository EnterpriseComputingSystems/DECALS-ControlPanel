use std::collections::HashMap;

use conrod::{Positionable, Widget, UiCell, Ui};
use conrod::widget::{Canvas};

use DECALS_base::data::{DataManager, DataReference};

use super::Panel;
use super::super::components::container::Container;

const CONT_BTNS: usize = 5;
const CONT_BTN_LABELS: [&str; 5] = ["", "", "", "Next", "Prev"];


widget_ids! {
    pub struct SettingsPanelIDs {
        canvas,

    }
}

pub struct SettingsPanel {
    ids: SettingsPanelIDs,
    container: Container,
    settings: HashMap<String, (Vec<String>, DataReference)>
}

impl SettingsPanel {
    pub fn new(ui: &mut Ui, dm: &DataManager, top_border: bool, bottom_border: bool, settings_options: &HashMap<String, Vec<String>>)-> SettingsPanel {

        let labels = CONT_BTN_LABELS.to_vec().iter().map(|s| s.to_string()).collect();




        let mut sp = SettingsPanel{ids: SettingsPanelIDs::new(ui.widget_id_generator()),
            settings: HashMap::new(),
            container: Container::new(ui, CONT_BTNS, top_border, bottom_border, dm, labels, Box::new(|_, _| {}))};


        for (key, options) in settings_options.iter() {
            sp.add_setting(key.clone(), options.clone(), dm.get_reference(key));
        }

        return sp;
    }

    pub fn add_setting(&mut self, key: String, options: Vec<String>, dr: DataReference) {
        self.settings.insert(key, (options, dr));
    }

}

impl Panel for SettingsPanel {

    fn build(&mut self, ui: &mut UiCell, base_canvas: Canvas) {

        let sub_canvas = self.container.build(ui, base_canvas);
        sub_canvas.set(self.ids.canvas, ui);

        for (key, &(ref options, ref dr)) in self.settings.iter() {

        }

    }
}
