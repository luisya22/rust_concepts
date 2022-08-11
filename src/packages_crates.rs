// A package is one or more crates that provide a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates.
//
// A crate can be a binary crate or a library crate. Binary crates are programs you can compile
// to an executable that you can run, such as a command-line program or a server. They must have
// a function called main that defines what happens when the executable runs. All the crates we’ve
// created so far have been binary crates.
//
// Library crates don’t have a main function, and they don’t compile to an executable.
// They define functionality intended to be shared with multiple projects. For example,
// the rand crate we used in Chapter 2 provides functionality that generates random numbers.

