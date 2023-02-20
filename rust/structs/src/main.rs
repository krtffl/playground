fn main() {
    let user1 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };

    // must be mutable!
    user2.email = String::from("example2@example.com");

    let user3 = User {
        email: String::from("example3@example.com"),
        username: use1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    //same as
    let user3_1 = User {
        email: String::from("example3@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // the struct is not owning all their data, so it will fail
    // for those cases we need lifetime specifiers
    let wrong_user = User {
        email: "example@example.com",
        ..user3
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
