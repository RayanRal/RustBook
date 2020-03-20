
struct User {
    username: String,
    email: String,
    count: u64
}

struct UserTuple(String, String, u64);

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        return self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u64) -> Rectangle {
        return Rectangle {width: size, height: size }
    }
}

fn create() {
    let user1 = User {
        username: String::from("user"),
        email: String::from("some_mail"),
        count: 1
    };

    let rect = Rectangle { width: 30, height: 50 };

}

fn create_user(name: String, email: String) -> User {
    return User {
        username: name,
        email,
        count: 0
    }
}

fn calc_area(r: &Rectangle) -> u64 {
    return r.width * r.height
}
