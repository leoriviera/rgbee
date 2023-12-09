#[derive(Default, Debug)]
pub struct Colour {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Colour {
    pub fn add(&mut self, c: &Self) {
        self.red += c.red;
        self.green += c.green;
        self.blue += c.blue;
    }
}
