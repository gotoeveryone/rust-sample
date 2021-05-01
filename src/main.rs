struct User {
    id: i32,
    name: String,
    flag: Option<bool>,
}

impl User {
    fn echo(&self) {
        println!("{}, {}, {}", self.id, self.name, self.flag.unwrap())
    }

    fn info(self) {
        println!("{}", &self.name.as_str())
    }
}

fn main() {
    let x = 9;
    let y = 12;
    let z = false;
    println!("Int, {}", x + y);
    println!("Bool, {}", !z);
    for i in 1..=5 {
        println!("index is {}", i);
    }
    
    let user = User { id: 1, name: "test".to_string(), flag: Some(false) };
    let user2 = user;
    // user.echo(); error: after move
    sub(&user2);

    let v = 9;
    let w = v;
    println!("v = {}, x = {}", v, w);

    user2.info()
}

fn sub(user: &User) {
    user.echo();
}
