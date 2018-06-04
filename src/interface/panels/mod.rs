
pub mod console;
pub mod basic_controls_panel;
pub mod full_image;
pub mod settings_panel;

use conrod::{UiCell};
use conrod::widget::Canvas;


pub trait Panel {
    fn build(&mut self, ui: &mut UiCell, base_canvas: Canvas);
}
