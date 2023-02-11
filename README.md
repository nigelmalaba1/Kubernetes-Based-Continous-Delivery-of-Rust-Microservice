# Kubernetes-Based-Continous-Delivery-of-Rust-Microservice
This project contains a machine to machine use case where a Rust Microservice communicates with a Flaks API to get data, and showcases Kubernetes Based Continous Delivery of a Rust Microservice 

##Configuration Steps

1. Create a Virtual Environment

The purpose of virtual environments is to create a self-contained environment for each of your projects, allowing you to manage dependencies, libraries, and versions separately for each project.

    `python3 -m venv rustenv`

    `source rustenv/bin/activate`

    `cd rustenv`

2. Install Rust
Go to https://rustup.rs/ and run the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

    Run `source "$HOME/.cargo/env"` to configure your current shell.

3. create new project
The cargo tool is the default package manager for Rust and provides an easy way to manage dependencies and build projects.

    Run `cargo new` (project name) (my Eg: `cargo new hello`)

This will create a binary (application) `microservice` package

4. Create `main.rs` and `lib.rs` files in the src project

    `touch main.rs` and `touch lib.rs` 

5. Run `Cargo build`   
This is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The cargo build command can be run from the root directory of the project.

5. Set up Cargo.toml to determine the dependencies and build configuration of the project.