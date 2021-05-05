#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    flag: Option<bool>,
}

impl User {
    fn echo(&self) {
        println!("{:?}", self)
    }

    fn info(self) {
        println!("{}", &self.name.as_str())
    }
}

fn sub(user: &User) {
    user.echo();
}

pub fn print_user_info() {
    let user = User {
        id: 1,
        name: "test".to_string(),
        flag: Some(false),
    };
    let user2 = user;
    // user.echo(); error: after move
    sub(&user2);

    let v = 9;
    let w = v;
    println!("v = {}, x = {}", v, w);

    user2.info();
}
