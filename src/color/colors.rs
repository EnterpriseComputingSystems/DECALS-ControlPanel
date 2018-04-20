
pub use conrod::color::Color;

pub type Pallette = Vec<Color>;


pub const RED:   Color = Color::Rgba(0.8, 0.0, 0.0, 1.0);
pub const GREEN: Color = Color::Rgba(0.0, 0.8, 0.0, 1.0);
pub const SPACEBLUE: Color = Color::Rgba(0.2, 0.2, 0.8, 1.0);
pub const BLACK: Color = Color::Rgba(0.0, 0.0, 0.0, 1.0);


pub const NO_ALERT: [Color; 8] = [Color::Rgba(1.0, 0.6, 0.0, 1.0),
                                    Color::Rgba(0.8, 0.6, 0.8, 1.0),
                                    Color::Rgba(0.6, 0.6, 0.8, 1.0),
                                    Color::Rgba(0.8, 0.4, 0.4, 1.0),
                                    Color::Rgba(1.0, 0.8, 0.6, 1.0),
                                    Color::Rgba(0.6, 0.6, 1.0, 1.0),
                                    Color::Rgba(1.0, 0.6, 0.4, 1.0),
                                    Color::Rgba(0.8, 0.4, 0.6, 1.0)];

pub const RED_ALERT: [Color; 8] = [Color::Rgba(1.0, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.8, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.7, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.6, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.5, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.45, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.4, 0.0, 0.0, 1.0),
                                    Color::Rgba(0.3, 0.0, 0.0, 1.0)];
