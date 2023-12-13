// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {

    // TODO: Fix these use statements
    use self::fruits::{PEAR as FruitPear, APPLE as FruitApple};
    use self::veggies::{CUCUMBER as VeggieCucumber, CARROT as VeggieCarrot};

    pub const fruit: &'static str = FruitPear;
    pub const veggie: &'static str = VeggieCucumber;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
