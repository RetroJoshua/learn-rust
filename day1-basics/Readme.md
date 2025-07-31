## Step 1: Installation in WINDOWS
Download: https://www.rust-lang.org/tools/install and Install

## Step 2: Verify the installation
```rustc --version```

## Step 3: Create project directory
```
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

## Step 4: Writing and Running a Rust Program

 Filename: main.rs 
```
fn main() {
    println!("Hello, world!");
}
```
On Windows, enter the command .\main instead of ./main:

```
> rustc main.rs
> .\main
Hello, world!
```

# Introduction to Cargo

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

check the cargo version

```
cargo --version
```

## Creating a project with cargo

```
$ cargo new hello_cargo
$ cd hello_cargo
```

The first command creates a new directory and project called hello_cargo. We’ve named our project hello_cargo, and Cargo creates its files in a directory of the same name.

Go into the hello_cargo directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

## Building and Running a Cargo Project

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:

```
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```
We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:
```
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

## Building for Release

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug. 

