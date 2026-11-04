#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 32,
        height: 1,
    };

    let other_rect = Rectangle {
        width: 2,
        height: 5,
    };

    rect1.set_to_max(other_rect);
}
