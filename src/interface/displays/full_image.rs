
use conrod::image;
use conrod::{Positionable, Widget, UiCell, Ui};
use conrod::widget::{Canvas, Image};

use DECALS_base::data::DataManager;

use super::Display;
use super::super::components::container::Container;

const CONT_BTNS: usize = 6;
const CONT_BTN_LABELS: [&str; 6] = ["", "", "", "", "", ""];


widget_ids! {
    pub struct FullImageIDs {
        canvas,
        image
    }
}

pub struct FullImage {
    ids: FullImageIDs,
    image: image::Id,
    container: Container
}

impl FullImage {
    pub fn new(ui: &mut Ui, dm: &DataManager, top_border: bool, bottom_border: bool, image: image::Id)-> FullImage {

        let labels = CONT_BTN_LABELS.to_vec().iter().map(|s| s.to_string()).collect();

        FullImage{ids: FullImageIDs::new(ui.widget_id_generator()),
                image,
                container: Container::new(ui, CONT_BTNS, top_border, bottom_border, dm, labels, Box::new(|_| {}))}
    }

}

impl Display for FullImage {

    fn build(&mut self, ui: &mut UiCell, base_canvas: Canvas) {

        let sub_canvas = self.container.build(ui, base_canvas);
        sub_canvas.set(self.ids.canvas, ui);

        Image::new(self.image).parent(self.ids.canvas)
            .middle_of(self.ids.canvas)
            .set(self.ids.image, ui);
    }
}
