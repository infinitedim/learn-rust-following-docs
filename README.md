
# Learn Rust Following Docs

My repository for learning rust programming language following official documentation


## Installation

- Install [Rust](https://www.rust-lang.org/tools/install) in your system

- Install in [64bit Windows](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

- Install in [32bit Windows](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

- Install in WSL/Mac/Linux
```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Test Rust and cargo in cli
```bash
  // Test rustc compiler
  rustc --version

  // Test cargo package manager
  cargo --version
```

- If installation successful version will be printed on your terminal / command line like this 
```bash
  // rustc
  rustc 1.70.0 (90c541806 2023-05-31)

  // cargo
  cargo 1.70.0 (ec8a8a0ca 2023-04-25)
```
## Run this repo in your computer

Clone the project

```bash
  // use https
  git clone https://github.com/infinitedim/learn-rust-following-docs

  // use ssh
  git clone git@github.com:infinitedim/learn-rust-following-docs.git
```

Go to the project directory

```bash
  cd learn-rust-following-docs
```

Run hello world

```bash
  // compile
  rustc main.rs

  // execute file
  ./main
```


## Commit

how to commit your changes

- Fork and clone this project, see how to clone this project above
- Create new branch
```bash
  git branch feat/your-features-branch
```
- Checkout to your features branch
```bash
  git checkout feat/your-features-branch
```
- Make a changes
- Add your changes
```bash
  git add .
```
- Commit your changes
```bash
  git commit -m "feat: your messege here"
```
- push your changes
```bash
git push origin your-features-branch
```
- Open pull request
- and done, wait until me approve your changes
## My Learning resource

 - [The Rust Programming Book](https://doc.rust-lang.org/book/)
 - [Rust Standart Library](https://doc.rust-lang.org/std/index.html)
 - [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
 - [Rustc Book](https://doc.rust-lang.org/rustc/index.html)
  - [Compiler Error Index](https://doc.rust-lang.org/error-index.html)

## License

This repo is unlicensed, please see official [Rust License](https://www.rust-lang.org/policies/licenses)


## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.

