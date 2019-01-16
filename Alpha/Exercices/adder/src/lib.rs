#[cfg(test)]
mod tests {
    use super::*;    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rectangle_can_hold_test() {
        let larger = Rectangle {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: larger.width - 1,
            height: larger.height - 1,
            ..larger
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
}


pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}