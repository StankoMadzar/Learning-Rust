trait Bite {
    fn bite(&mut self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.amount_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
}


fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);


    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
    bunny_nibbles(&mut grapes);

    
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(&mut self) {
        self.percent_left *= 0.8;
    }
}