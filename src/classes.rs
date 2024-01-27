// Rectangle Class
pub struct Rectangle {
    /// Rectangle classes with width and hight atrribute.
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
// Circle Class
pub struct Circle {
    pub radius: f64,
}
impl Circle {
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
// Triangle class
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Triangle {
    pub fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}
