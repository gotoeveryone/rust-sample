mod random;
mod sensor;
mod user;

fn main() {
    let x = 9;
    let y = 12;
    let z = false;
    println!("Int, {}", x + y);
    println!("Bool, {}", !z);
    for i in 1..=5 {
        println!("index is {}", i);
    }

    user::print_user_info();

    sensor::print_sensor_info();

    let v1 = 96;
    let v2 = i64::from(v1);

    use std::convert::TryFrom;

    let v3 = i32::try_from(v2);
    println!("v1 = {}, v2 = {}, v3 = {}", v1, v2, v3.unwrap());

    random::print_random()
}
