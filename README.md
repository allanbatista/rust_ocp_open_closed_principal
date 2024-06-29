# Open-Closed Principle in Rust

This project demonstrates the Open-Closed Principle (OCP) in Rust. It contains two binaries: `ocp` and `not_ocp`.

## OCP

The `ocp` binary demonstrates the correct application of the OCP. It defines a `LogIn` trait and three structs (`UserAdmin`, `UserCommon`, and `User`) that implement this trait. The `login` function takes a reference to a `LogIn` trait object, allowing it to work with any struct that implements `LogIn`.

This design is open for extension (we can add new structs that implement `LogIn`) and closed for modification (adding a new struct doesn't require us to modify the existing code).

You can run this binary with `cargo run --bin ocp`.

## Not OCP

The `not_ocp` binary demonstrates a violation of the OCP. It defines a `NotOcpUser` struct with a `user_type` field. The `login` function uses a `match` statement to determine the behavior based on the `user_type`.

This design is not open for extension (adding a new user type requires us to modify the `login` function) and not closed for modification (the `login` function has to change whenever a new user type is added).

You can run this binary with `cargo run --bin not_ocp`.

## Running the Project

To run the project, you will need Rust and Cargo installed on your machine. You can run the binaries with the following commands:

```bash
cargo run --bin ocp
cargo run --bin not_ocp
```