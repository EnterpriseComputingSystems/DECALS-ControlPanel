

pub use conrod::color::Color;

pub type Pallette = Vec<Color>;

#[allow(dead_code)]
pub const SPACEBLUE: Color = Color::Rgba(0.2, 0.2, 0.8, 1.0);


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


pub const BLUE_ALERT: [Color; 8] = [Color::Rgba(0.0, 0.0, 1.0, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.8, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.7, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.6, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.5, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.45, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.4, 1.0),
                                    Color::Rgba(0.0, 0.0, 0.3, 1.0)];


pub const YELLOW_ALERT: [Color; 5] = [Color::Rgba(0.4, 0.26, 0.02, 1.0),
                                    Color::Rgba(0.6, 0.4, 0.03, 1.0),
                                    Color::Rgba(0.8, 0.53, 0.05, 1.0),
                                    Color::Rgba(1.0, 0.66, 0.10, 1.0),
                                    Color::Rgba(1.0, 0.85, 0.4, 1.0)];
