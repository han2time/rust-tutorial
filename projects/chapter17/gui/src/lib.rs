// 17.2.1
pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for components in self.components.iter() {
            components.draw();
        }
    }
}
/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for components in self.components.iter() {
            components.draw();
        }
    }
}
*/

// 17.2.2
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 실제로 버튼을 그리는 코드
    }
}