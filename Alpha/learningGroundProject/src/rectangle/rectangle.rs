struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}


impl Rectangle {
    fn area(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

}