


//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Rounded Button
pub mod rounded_button {
    use conrod::{self, widget, Colorable, Labelable, Point, Positionable, Widget, Rect, Scalar, Range, Sizeable};
    use conrod::widget::primitive::shape::oval::Circumference;

    use std::f64::consts::PI;


    /// The type upon which we'll implement the `Widget` trait.
    #[derive(WidgetCommon)]
    pub struct RoundedButton<'a> {
        /// An object that handles some of the dirty work of rendering a GUI. We don't
        /// really have to worry about it.
        #[conrod(common_builder)]
        common: widget::CommonBuilder,
        /// Optional label string for the button.
        maybe_label: Option<&'a str>,
        /// See the Style struct below.
        style: Style,
        /// Whether the button is currently enabled, i.e. whether it responds to
        /// user input.
        enabled: bool,

        radii: [Scalar; 4]
    }

    // We use `#[derive(WidgetStyle)] to vastly simplify the definition and implementation of the
    // widget's associated `Style` type. This generates an implementation that automatically
    // retrieves defaults from the provided theme in the following order:
    //
    // 1. If the field is `None`, falls back to the style stored within the `Theme`.
    // 2. If there are no style defaults for the widget in the `Theme`, or if the
    //    default field is also `None`, falls back to the expression specified within
    //    the field's `#[conrod(default = "expr")]` attribute.

    /// Represents the unique styling for our CircularButton widget.
    #[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
    pub struct Style {
        /// Color of the button.
        #[conrod(default = "theme.shape_color")]
        pub color: Option<conrod::Color>,
        /// Color of the button's label.
        #[conrod(default = "theme.label_color")]
        pub label_color: Option<conrod::Color>,
        /// Font size of the button's label.
        #[conrod(default = "theme.font_size_medium")]
        pub label_font_size: Option<conrod::FontSize>,
        /// Specify a unique font for the label.
        #[conrod(default = "theme.font_id")]
        pub label_font_id: Option<Option<conrod::text::font::Id>>,
    }

    // We'll create the widget using a `Circle` widget and a `Text` widget for its label.
    //
    // Here is where we generate the type that will produce these identifiers.
    widget_ids! {
        struct Ids {
            polygon,
            text,
        }
    }

    /// Represents the unique, cached state
    pub struct State {
        ids: Ids,
    }

    impl<'a> RoundedButton<'a> {

        /// Create a button context to be built upon.
        pub fn new() -> Self {
            RoundedButton {
                common: widget::CommonBuilder::default(),
                style: Style::default(),
                maybe_label: None,
                enabled: true,
                radii: [0.0, 0.0, 0.0, 0.0]
            }
        }

        pub fn rounded_left(radius: Scalar) -> Self {
            RoundedButton {
                common: widget::CommonBuilder::default(),
                style: Style::default(),
                maybe_label: None,
                enabled: true,
                radii: [0.0, radius, radius, 0.0]
            }
        }

        /// Specify the font used for displaying the label.
        #[allow(dead_code)]
        pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
            self.style.label_font_id = Some(Some(font_id));
            self
        }

        /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
        /// other Conrod configs, this returns self for chainability.
        #[allow(dead_code)]
        pub fn enabled(mut self, flag: bool) -> Self {
            self.enabled = flag;
            self
        }

    }

    /// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
    /// documentation for more details.
    impl<'a> Widget for RoundedButton<'a> {
        /// The State struct that we defined above.
        type State = State;
        /// The Style struct that we defined using the `widget_style!` macro.
        type Style = Style;
        /// The event produced by instantiating the widget.
        ///
        /// `Some` when clicked, otherwise `None`.
        type Event = Option<()>;

        fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
            State { ids: Ids::new(id_gen) }
        }

        fn style(&self) -> Self::Style {
            self.style.clone()
        }

        /// Optionally specify a function to use for determining whether or not a point is over a
        /// widget, or if some other widget's function should be used to represent this widget.
        ///
        /// This method is optional to implement. By default, the bounding rectangle of the widget
        /// is used.
        fn is_over(&self) -> widget::IsOverFn {
            use conrod::graph;
            use conrod::Theme;
            // Stolen from 'RoundedRectangle' implementation
            pub fn is_over_widget(widget: &graph::Container, point: Point, _: &Theme) -> widget::IsOver {
                widget
                    .unique_widget_state::<RoundedButton>()
                    .map(|widget| widget.state.ids.polygon.into())
                    .unwrap_or_else(|| widget.rect.is_over(point).into())
            }
            is_over_widget
        }

        /// Update the state of the button by handling any input that has occurred since the last
        /// update.
        fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
            let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;

            let (color, event) = {
                let input = ui.widget_input(id);

                // If the button was clicked, produce `Some` event.
                let event = input.clicks().left().next().map(|_| ());

                let color = style.color(&ui.theme);
                let color = input.mouse().map_or(color, |mouse| {
                    if mouse.buttons.left().is_down() {
                        color.clicked()
                    } else {
                        color.highlighted()
                    }
                });

                (color, event)
            };

            // First, we'll draw the **Circle** with a radius that is half our given width.
            let points = points(rect, self.radii);
            let (x, y, w, h) = rect.x_y_w_h();
            widget::Polygon::fill(points)
                .x_y(x, y)
                .w_h(w, h)
                .parent(id)
                .color(color)
                .graphics_for(id)
                .set(state.ids.polygon, ui);

            // Generate a default label if there isn't one specified using the button's index as a
            // seed for a simple encoder
            let mut tmp: String;
            let label: &str;
            if let Some(lab) = self.maybe_label {
                label = lab;
            } else {
                let gen = 53647699;
                let mut num = id.index() << 16;
                num = num ^ gen;
                num = num % 1000000;
                tmp = num.to_string();
                tmp.insert(2, '-');
                label = tmp.as_str();
            }

            // Now we'll instantiate our label using the **Text** widget.
            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new(label)
                .and_then(font_id, widget::Text::font_id)
                .bottom_right_with_margins_on(id, 7.0, 7.0 + self.radii[3]) //for bottom right corner keep text in further
                .font_size(font_size)
                .right_justify()
                .graphics_for(id)
                .color(label_color)
                .set(state.ids.text, ui);

            event
        }

    }

    /// Provide the chainable color() configuration method.
    impl<'a> Colorable for RoundedButton<'a> {
        fn color(mut self, color: conrod::Color) -> Self {
            self.style.color = Some(color);
            self
        }
    }

    /// Provide the chainable label(), label_color(), and label_font_size()
    /// configuration methods.
    impl<'a> Labelable<'a> for RoundedButton<'a> {
        fn label(mut self, text: &'a str) -> Self {
            self.maybe_label = Some(text);
            self
        }
        fn label_color(mut self, color: conrod::Color) -> Self {
            self.style.label_color = Some(color);
            self
        }
        fn label_font_size(mut self, size: conrod::FontSize) -> Self {
            self.style.label_font_size = Some(size);
            self
        }
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Stolen from RoundedRectangle

    /// An iterator yielding the outer points of a `RoundedRectangle`
    #[derive(Clone)]
    pub struct Points {
        corner_rect: [Rect; 4],
        corner_index: usize,
        corner_points: Circumference,
    }

    const CORNER_RADIANS: Scalar = PI * 0.5;
    const CORNER_RESOLUTION: usize = 20;

    /// Produce an iterator yielding the outer points of a rounded rectangle.
    ///
    /// - `rect` describes the location and dimensions
    /// - `radius` describes the radius of the corner circles.
    pub fn points(rect: Rect, radii: [Scalar; 4]) -> Points {
        // First corner is the top right corner.

        let rects = [

        Rect {
            x: Range { start: rect.x.end - radii[0] * 2.0, end: rect.x.end },
            y: Range { start: rect.y.end - radii[0] * 2.0, end: rect.y.end },
        },

        Rect {
            x: Range { start: rect.x.start, end: rect.x.start + radii[1] * 2.0},
            y: Range { start: rect.y.end - radii[1] * 2.0, end: rect.y.end },
        },

        Rect {
            x: Range { start: rect.x.start, end: rect.x.start + radii[2] * 2.0},
            y: Range { start: rect.y.start, end: rect.y.start + radii[2] * 2.0},
        },

        Rect {
            x: Range { start: rect.x.end - radii[0] * 2.0, end: rect.x.end },
            y: Range { start: rect.y.start, end: rect.y.start + radii[2] * 2.0},
        }];

        let corner = Circumference::new_section(rects[0], CORNER_RESOLUTION, CORNER_RADIANS);
        Points {
            corner_rect: rects,
            corner_index: 0,
            corner_points: corner,
        }
    }

    impl Iterator for Points {
        type Item = Point;
        fn next(&mut self) -> Option<Self::Item> {
            let Points {
                ref mut corner_rect,
                ref mut corner_index,
                ref mut corner_points,
            } = *self;
            loop {
                if let Some(point) = corner_points.next() {
                    return Some(point);
                }

                if *corner_index == 3 {
                    return None;
                }

                *corner_index += 1;
                let offset_radians = *corner_index as Scalar * CORNER_RADIANS;
                *corner_points = Circumference::new_section(corner_rect[*corner_index], CORNER_RESOLUTION, CORNER_RADIANS)
                    .offset_radians(offset_radians);
            }
        }
    }
}
