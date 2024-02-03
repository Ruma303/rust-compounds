struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

    fn main() {
        let mut user1 : User = User {
            email: String::from("user@1.com"),
            username: String::from("user1"),
            active: true,
            sign_in_count: 1,
        };
        user1.active = false; //* ok

        let user_email = user1.email;
        println!("{user_email}"); // user@1.com

    }

