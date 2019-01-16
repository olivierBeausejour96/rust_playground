#[cfg(test)]
mod rectangle_tests {
    use adder::*;

    #[test]
    fn can_hold() {
        let larger = Rectangle {
            x: 0,
            y: 0,
            width: 10,
            height: 10
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