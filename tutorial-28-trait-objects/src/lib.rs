pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // dyn means any type that implements the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button with width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}
