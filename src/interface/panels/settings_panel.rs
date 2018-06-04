use std::collections::HashMap;

use conrod::{Positionable, Widget, UiCell, Ui};
use conrod::widget::{Canvas};
use conrod::{Labelable, Sizeable};

use DECALS_base::data::{DataManager, DataReference};

use super::Panel;
use super::super::components::container::Container;
use super::super::widgets::rounded_button::RoundedButton;

const CONT_BTNS: usize = 5;
const CONT_BTN_LABELS: [&str; 5] = ["", "", "", "Next", "Prev"];


widget_ids! {
    pub struct SettingsPanelIDs {
        canvas,
        settings_canvases[],
        settings_btns[]
    }
}

pub struct SettingsPanel {
    ids: SettingsPanelIDs,
    container: Container,
    settings: HashMap<String, (Vec<String>, DataReference)>
}

impl SettingsPanel {
    pub fn new(ui: &mut Ui, dm: &DataManager, top_border: bool, bottom_border: bool, settings_options: &HashMap<String, Vec<String>>)-> SettingsPanel {

        let mut num_btns = 0;
        let mut settings: HashMap<String, (Vec<String>, DataReference)> = HashMap::new();
        for (key, options) in settings_options.iter() {
            num_btns += options.len() + 1; //include header btn
            settings.insert(key.clone(), (options.clone(), dm.get_reference(key)));
        }

        let labels = CONT_BTN_LABELS.to_vec().iter().map(|s| s.to_string()).collect();
        let mut ids = SettingsPanelIDs::new(ui.widget_id_generator());

        ids.settings_canvases.resize(settings.len(), &mut ui.widget_id_generator());
        ids.settings_btns.resize(num_btns, &mut ui.widget_id_generator());


        let sp = SettingsPanel{ids,
            settings,
            container: Container::new(ui, CONT_BTNS, top_border, bottom_border, dm, labels, Box::new(|_, _| {}))};

        return sp;
    }

    pub fn update_settings(&mut self, ui: &mut UiCell, dm: &DataManager, settings_options: &HashMap<String, Vec<String>>) {
        let mut num_btns = 0;
        let mut settings: HashMap<String, (Vec<String>, DataReference)> = HashMap::new();
        for (key, options) in settings_options.iter() {
            num_btns += options.len() + 1; //include header btn
            settings.insert(key.clone(), (options.clone(), dm.get_reference(key)));
        }

        self.ids.settings_canvases.resize(settings.len(), &mut ui.widget_id_generator());
        self.ids.settings_btns.resize(num_btns, &mut ui.widget_id_generator());
    }

}

impl Panel for SettingsPanel {

    fn build(&mut self, ui: &mut UiCell, base_canvas: Canvas) {

        let sub_canvas = self.container.build(ui, base_canvas);
        sub_canvas.set(self.ids.canvas, ui);

        let mut settings_idx = 0;
        let mut btn_idx = 0;
        for (key, &(ref options, ref dr)) in self.settings.iter() {

            if settings_idx == 0 {
                Canvas::new().parent(self.ids.canvas)
                    .w(150.0)
                    .mid_left_of(self.ids.canvas)
                    .set(self.ids.settings_canvases[settings_idx], ui);
            } else {
                Canvas::new().parent(self.ids.canvas)
                    .right_from(self.ids.settings_canvases[settings_idx - 1], 20.0)
                    .w(150.0)
                    .set(self.ids.settings_canvases[settings_idx], ui);
            }

            RoundedButton::new().label(key)
                .h(70.0)
                .mid_top_of(self.ids.settings_canvases[settings_idx])
                .set(self.ids.settings_btns[btn_idx], ui);
            btn_idx += 1;

            for option in options {
                for _press in RoundedButton::rounded_left(20.0)
                    .h(70.0)
                    .label(option)
                    .down_from(self.ids.settings_btns[btn_idx - 1], 5.0)
                    .set(self.ids.settings_btns[btn_idx], ui) {
                        dr.update_data(option.clone());
                    }

                btn_idx += 1;
            }



            settings_idx += 1;
        }

    }
}
