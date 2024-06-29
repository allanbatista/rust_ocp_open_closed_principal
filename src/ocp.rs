trait LogIn {
    fn log_in(&self);
}

struct UserAdmin {}

impl LogIn for UserAdmin {
    fn log_in(&self) {
        println!("Admin logged in");
    }
}

struct UserCommon {}

impl LogIn for UserCommon {
    fn log_in(&self) {
        println!("Common user logged in");
    }
}

struct User {}

impl LogIn for User {
    fn log_in(&self) {
        println!("User logged in");
    }
}

fn login(user: &dyn LogIn) {
    user.log_in();
}


fn main() {
    // The UserAdmin, UserCommon, and User classes each implement the LogIn trait.
    // This is an application of the Open-Closed Principle (OCP), which states that
    // software entities (classes, modules, functions, etc.) should be open for extension,
    // but closed for modification.
    //
    // By having each user type implement the LogIn trait, we can add new user types
    // in the future without modifying the existing user types or the login function.
    // Each new user type would simply need to implement the LogIn trait.
    //
    // This makes our code more flexible and easier to maintain, as changes to one
    // user type won't affect the others.

    let admin = UserAdmin {};
    login(&admin);

    let common = UserCommon {};
    login(&common);

    let user = User {};
    login(&user);
}
