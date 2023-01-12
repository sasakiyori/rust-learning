pub trait Fruit {
    fn color(&self) -> &str {
        "no color"
    }
    fn taste(&self) -> &str {
        "no taste"
    }
    fn compare(&self, other: &Self) -> bool {
        println!(
            "my taste: {}, other's taste: {}",
            self.taste(),
            other.taste()
        );
        println!(
            "my color: {}, other's color: {}",
            self.color(),
            other.color()
        );
        true
    }
}
