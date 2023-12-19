use hsl::HSL;

#[derive(Clone, Default, Debug)]
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

    pub fn from_rgba8(rgba8: [u8; 4]) -> Self {
        Colour {
            red: rgba8[0] as usize,
            green: rgba8[1] as usize,
            blue: rgba8[2] as usize,
        }
    }

    pub fn from_hsl(c: HSL) -> Self { 
        let (r, g, b) = c.to_rgb();

        Colour::from_rgba8([r, g, b, 0])
    }
}
