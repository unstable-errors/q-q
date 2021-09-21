<p align="center">
 <h1 align="center">q>q</h1>
 <h3 align="center">Quantity > Quality</h3>
 <h6 align="center">q>q documentation</p>
  <p align="center">
    <img src="https://img.shields.io/github/repo-size/unstable-errors/q-q?style=for-the-badge"/>
    <img src="https://img.shields.io/github/languages/top/unstable-errors/q-q?style=for-the-badge"/>
    <img src="https://img.shields.io/github/downloads/unstable-errors/q-q/total?style=for-the-badge"/>
    <img src="https://img.shields.io/github/workflow/status/unstable-errors/q-q/Rust?style=for-the-badge"/>
    <img src="https://img.shields.io/github/commit-activity/m/unstable-errors/q-q?style=for-the-badge"/>
  </p>


# Welcome!
This is a simple guide of how to contribute to q>q! There are many ways to contribute to this!
To contribute with code, you will need to know [rust](https://rust-lang.org). To learn it you can view the rust book (online, free)!

## Contributing code via pull request
Before we start, there are minimal comments (adding some more soon), so you're just stuck with this and your skills (more documentation coming soon)

For this example, we will add a new app:

1. Open your local clone of your fork (make sure its on a new branch)
2. Create a new file named *name*.rs in the src/*type*/ directory
3. Put your code into it and change "fn main()" to "pub fn *name*()"
4. Open src/*type*.rs and add "pub mod *name*;" (if it doesnt already exist make it)
5. Go to src/main.rs and to the variable *type*list add the name of your program to it
6. Go to src/menu_handler.rs and add this above the else statement (replace name and type to the name and type of the program)
```rust
else if app == "name"
{
    type::name::name();
}
```
7. Run `cargo run` to test everything
8. If it works, add, commit and push everything to your fork
9. Make a pull request across your fork's branch and the main branch for the main repo
10. Wait for a review

Replace anything in italics with the actual name.
The types are `other, games, apps`
