use rand::Rng;

fn random_number() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..=100);
}
fn main() {
    let target = random_number();
    let mut guess = String::new();
    println!("Please enter your guess:");
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    if guess > target {
        println!("Too big!");
    } else if guess < target {
        println!("Too small!");
    } else {
        println!("You win!");
    }
}