struct NotOcpUser {
    user_type: String,
}

impl NotOcpUser {
    fn new(user_type: String) -> Self {
        Self { user_type }
    }

    fn get_user_type(&self) -> &str {
        &self.user_type
    }
}

fn login(user: NotOcpUser) {
    match user.get_user_type() {
        "admin" => println!("Admin logged in"),
        "common" => println!("Common user logged in"),
        _ => println!("User logged in"),
    }
}

fn main() {
    // The NotOcpUser struct uses a user_type field to differentiate between different types of users.
    // This is a violation of the Open-Closed Principle (OCP), which states that
    // software entities (classes, modules, functions, etc.) should be open for extension,
    // but closed for modification.
    //
    // In this case, if we want to add a new user type, we would need to modify the login function
    // to add a new match arm for the new user type. This makes our code less flexible and harder to maintain,
    // as changes to the user types directly affect the login function.
    let admin = NotOcpUser::new("admin".to_string());
    login(admin);

    let common = NotOcpUser::new("common".to_string());
    login(common);

    let user = NotOcpUser::new("user".to_string());
    login(user);
}