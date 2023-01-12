use pkg::fruit::Fruit;
use pkg::meat::Meat;

mod pkg;

struct Banana {}
impl Fruit for Banana {}

struct Pineapple {}
impl Fruit for Pineapple {
    fn compare(&self, other: &Self) -> bool {
        println!("pineapple derive");
        println!(
            "other's color and taste: {}, {}",
            other.color(),
            other.taste()
        );
        false
    }
}

struct Apple {
    color: String,
    taste: String,
}
impl Fruit for Apple {
    fn color(&self) -> &str {
        &self.color
    }
    fn taste(&self) -> &str {
        &self.taste
    }
    fn compare(&self, other: &Self) -> bool {
        println!("apple derive");
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
        self.taste == other.taste && self.color == other.color
    }
}

impl Meat for Apple {
    fn smell(&self) {}
}

fn describe<T: Fruit>(fruit: T) {
    println!("describe: {} {}", fruit.color(), fruit.taste());
}

struct Shop<T> {
    s: T,
}
impl<T: Fruit + Meat> Shop<T> {
    fn open(&self) {
        self.s.color();
        self.s.taste();
        self.s.smell();
    }
}

fn main() {
    let b1 = Banana {};
    let b2 = Banana {};
    println!("banana compare result: {}", b1.compare(&b2));

    let p1 = Pineapple {};
    let p2 = Pineapple {};
    println!("pineapple compare result: {}", p1.compare(&p2));

    let a1 = Apple {
        color: String::from("green"),
        taste: String::from("good"),
    };
    let a2 = Apple {
        color: String::from("red"),
        taste: String::from("good"),
    };
    println!("apple compare result: {}", a1.compare(&a2));

    describe(b1);
    describe(p1);
    describe(a1);

    let s1 = Shop {
        s: Apple {
            color: String::from("green"),
            taste: String::from("good"),
        },
    };
    s1.s.color();
    s1.s.taste();
    s1.s.smell();
    Meat::smell(&s1.s);
    s1.open();

    // Banana can be used as the generic of Shop, even though Banana does not implement Meat
    // this is very different with GO/C++
    let s2 = Shop { s: Banana {} };
    s2.s.color();
    s2.s.taste();
    // these functions below are not allowed.
    // s2.s.smell();
    // s2.open();
}
