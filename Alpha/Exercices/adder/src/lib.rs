#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn fails() {
        panic!("Expected to fail!");
    }

    fn fails_differently(){
        assert_eq!(2+2, 6);
    }
}
