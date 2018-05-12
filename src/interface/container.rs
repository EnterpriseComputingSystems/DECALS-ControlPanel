

extern crate rand;

use std::sync::Arc;

use DECALS_base::support::alert::Alert;

use super::vertical_menu::VerticalMenu;
use super::super::color::{ColorScheme, IteratedScheme};
use super::super::color::colors;
use super::super::color::colors::{Color, Pallette};

use conrod;
use conrod::{Colorable, Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;
use conrod::widget::primitive::shape::rectangle::Rectangle;

const BORDER_RECT_WIDTH: Scalar = 20.0;
const BTN_GAP: Scalar = 2.0;
const VERT_MENU_WIDTH: Scalar = 70.0;


widget_ids! {
    pub struct ContainerIDs {
        canvas,
        top_rect

    }
}

pub struct Container {
    ids: ContainerIDs,
    vert_menu: VerticalMenu,
    top_border: bool,
    bottom_border: bool
}

impl Container {
    pub fn new(ui: &mut Ui, num_btns: usize, top_border: bool, bottom_border: bool)-> Container {
        Container{ids: ContainerIDs::new(ui.widget_id_generator()),
            vert_menu: VerticalMenu::new(ui, num_btns),
            top_border, bottom_border}
    }



    pub fn build(&self, ui: &mut UiCell, alert_status: Alert, base_canvas: Canvas) {


        base_canvas.color(conrod::color::RED);
        base_canvas.set(self.ids.canvas, ui);
        let base_dim = base_canvas.get_wh(ui).unwrap();
        let mut menu_height = base_dim[1];

        if self.top_border {
            menu_height -= BORDER_RECT_WIDTH + BTN_GAP;
        }

        if self.bottom_border {
            menu_height -= BORDER_RECT_WIDTH + BTN_GAP;
        }

        let vm_canvas = Canvas::new().parent(self.ids.canvas)
                        .h(menu_height)
                        .w(VERT_MENU_WIDTH);

        // If just a top border, put in bottom
        // if self.top_border && !self.bottom_border {
            vm_canvas.bottom_left_of(self.ids.canvas);
        //
        // // If just a bottom border, put in bottom
        // } else if !self.top_border && self.bottom_border {
        //     vm_canvas.top_left_of(self.ids.canvas);
        //
        // // If both or neither borders, put in middle
        // } else {
            // vm_canvas.mid_left_of(self.ids.canvas);
        // }


        self.vert_menu.build(ui, alert_status, vm_canvas);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Alert buttons

        let mut alert_scheme: Arc<ColorScheme> = Arc::new(match alert_status {
            Alert::Normal=> IteratedScheme::new(colors::NO_ALERT.to_vec()),
            Alert::Yellow=> IteratedScheme::new(colors::YELLOW_ALERT.to_vec()),
            Alert::Blue=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
            Alert::Black=> IteratedScheme::new(colors::BLUE_ALERT.to_vec()),
            Alert::Red=> IteratedScheme::new(colors::RED_ALERT.to_vec())

        });

        if self.top_border {
            Rectangle::fill_with([base_dim[0], BORDER_RECT_WIDTH], Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
                .mid_top_of(self.ids.canvas)
                .set(self.ids.top_rect, ui);
        }

        if self.bottom_border {
            Rectangle::fill_with([base_dim[0], BORDER_RECT_WIDTH], Arc::get_mut(&mut alert_scheme).unwrap().get_next_color())
                .mid_bottom_of(self.ids.canvas)
                .set(self.ids.top_rect, ui);
        }

    }

}
