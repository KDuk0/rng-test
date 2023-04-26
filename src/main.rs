use rand::prelude::*;

fn main() {

    let mut rng = rand::thread_rng();

    for _i in 1..=5 {
        let random_number:u16 = rng.gen();
        println!("Generated the number: {}", random_number);

    }

}
