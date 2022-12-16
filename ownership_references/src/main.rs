// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn inspect (argument: &String) {
    if argument.ends_with("s") {
        println!("plural boi");
    } else {
        println!("singular boi");
    }
}

fn change(argument : &mut String) {
    if !argument.ends_with("s") {
        argument.push_str("s");
        println!("The argument is now {}", argument);
    }
}

fn eat (argument : String) -> bool {
    if argument.starts_with("b") && argument.contains("a") {
        true
    } else {
        false
    }
}

fn bedazzle (argument : &mut String) {
    (*argument) = String::from("sparkly");
}

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    }).to_owned();

    inspect(&arg);
    change(&mut arg);
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
