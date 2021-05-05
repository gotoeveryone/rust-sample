use oorandom::Rand32;

pub fn print_random() {
    println!("random = {}", Rand32::new(123).rand_u32())
}
