

extern crate rand;

use DECALS_base::support::alert::Alert;

use super::vertical_menu::VerticalMenu;
use super::super::color::ColorScheme;
use super::super::color::colors;

use conrod::{Positionable, Sizeable, Widget, UiCell, Scalar, Ui};
use conrod::widget::Canvas;
use conrod::widget::primitive::shape::rectangle::Rectangle;

const BORDER_RECT_WIDTH: Scalar = 20.0;
const BTN_GAP: Scalar = 2.0;
const VERT_MENU_WIDTH: Scalar = 70.0;
const CHILD_MARGIN: Scalar = 20.0;

const ANIMATION_FREQ: u32 = 2;


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
    bottom_border: bool,
    last_alert: Alert,
    cscheme: ColorScheme,
    ticks: u32
}

impl Container {
    pub fn new(ui: &mut Ui, num_btns: usize, top_border: bool, bottom_border: bool)-> Container {

        let mut vm_labels: Vec<String> = Vec::new();

        for i in 0..num_btns {
            vm_labels.push(String::new());
        }

        let vm_btn_handler = |btn: usize| {
            match btn {
                _=>()
            }
        };


        Container{ids: ContainerIDs::new(ui.widget_id_generator()),
            vert_menu: VerticalMenu::new(ui, num_btns, vm_labels, Box::new(vm_btn_handler)),
            top_border, bottom_border,
            cscheme: ColorScheme::new(colors::NO_ALERT.to_vec()),
            last_alert: Alert::Normal,
            ticks: 0}
    }



    pub fn build(&mut self, ui: &mut UiCell, alert_status: Alert, base_canvas: Canvas)-> Canvas {

        if alert_status != self.last_alert {
            self.last_alert = alert_status;
            self.cscheme = super::get_colorscheme(alert_status);
            self.ticks = 0;
        } else {
            self.cscheme.reset_to_start();
        }


        if alert_status != Alert::Normal {
            self.ticks = (self.ticks + 1) % ANIMATION_FREQ;

            if self.ticks == ANIMATION_FREQ - 1 {
                self.cscheme.rotate_start();
            }
        }


        base_canvas.set(self.ids.canvas, ui);
        let base_dim = base_canvas.get_wh(ui).unwrap();
        let mut menu_height = base_dim[1];

        if self.top_border {
            menu_height -= BORDER_RECT_WIDTH + BTN_GAP;
        }

        if self.bottom_border {
            menu_height -= BORDER_RECT_WIDTH + BTN_GAP;
        }

        //Add top border
        if self.top_border {
            Rectangle::fill_with([base_dim[0], BORDER_RECT_WIDTH], self.cscheme.get_next_color())
                .mid_top_of(self.ids.canvas)
                .set(self.ids.top_rect, ui);
        }


        // Add buttons
        let mut vm_canvas = Canvas::new().parent(self.ids.canvas)
                        .h(menu_height)
                        .w(VERT_MENU_WIDTH);


        // If just a top border, put in bottom
        // If just a bottom border, put in top
        // If both or neither borders, put in middle
        vm_canvas = match (self.top_border, self.bottom_border) {
            (true, false)=>vm_canvas.bottom_left_of(self.ids.canvas),
            (false, true)=>vm_canvas.top_left_of(self.ids.canvas),
            _=>vm_canvas.mid_left_of(self.ids.canvas)
        };


        self.vert_menu.build(ui, &mut self.cscheme, vm_canvas);

        // Add lower border
        if self.bottom_border {
            Rectangle::fill_with([base_dim[0], BORDER_RECT_WIDTH], self.cscheme.get_next_color())
                .mid_bottom_of(self.ids.canvas)
                .set(self.ids.top_rect, ui);
        }

        Canvas::new().parent(self.ids.canvas)
            .bottom_right_of(self.ids.canvas)
            .wh([base_dim[0] - VERT_MENU_WIDTH - CHILD_MARGIN, base_dim[1] - BORDER_RECT_WIDTH - CHILD_MARGIN])

    }

}
