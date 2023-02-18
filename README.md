<p align="center">
  <img src="./screenshots/rusp-cover.png" />
</p>

## Why it's name is RUSP?

It's a play on "Rust" and "curl" that could stand for "Rust Simple Protocol".

I developed this HTTP client app for the purpose of having fun and improving my Rust skills. While attempting to reverse-engineer the CURL application, I created a relatively simple application.

## Features:

- [x] Arguments.
- [x] Http requests.
  - [x] GET
  - [x] POST
  - [x] PUI
  - [x] PATCH
  - [x] DELETE
- [x] Pretty print response.
- [ ] Testing.

## Installation:

If for some reason you want to use this script follow the next steps.

> DISCLAIMER!!!
>
> - This set up is for linux users.
> - You need to have installed `cargo` on your machine.

1. Clone this repository:

```sh
git clone https://github.com/freddyvelarde/rusp
```

2. Move this project to .config file:

```sh
mv rusp ~/.config
# then
cd ~/.config/rusp
```

3. Just run this command in the `rusp`project directory:

```
cargo build
# or
cargo run
```

4. Copy and paste this line in your .zshrc if you're zsh user or .bashrc if you're bash user:

```bash
alias rusp="~/.config/rusp/target/debug/rusp --"
```

5. Finally run this command depending if you're bash or zsh user (or even another unix shell like fish)

```bash
# zsh users
source ~/.zshrc

# bash users
source ~/.bashrc
```

## Usage:

Run the help and version command:

```bash
rusp --help
rusp --version
```

```bash
rusp --url https://jsonplaceholder.typicode.com/users/1 --method GET
```

![get screen](./screenshots/get-req.png)

```bash
rusp --url https://jsonplaceholder.typicode.com/posts --method POST --body '{"title": "title post", "content": "content post"}' --header 'Content-Type: application/json'
```

![post screen](./screenshots/post-req.png)

```bash
rusp --url https://jsonplaceholder.typicode.com/posts/1 --method PUT --body '{"id": 1, "title": "foo", "body": "bar", "userId": 1}' --header 'Content-Type: application/json'
```

![post screen](./screenshots/put-req.png)

```bash
rusp --url https://jsonplaceholder.typicode.com/posts/1 --method PATCH -b '{"title": "foo"}' --header 'Content-Type: application/json'
```

![post screen](./screenshots/patch-req.png)

```bash
rusp --url https://jsonplaceholder.typicode.com/posts/1 --method DELETE
```

Multiheaders:

```bash
rusp -u https://jsonplaceholder.typicode.com/posts -m POST -h 'Content-Type: application/json' -h 'access-token: your secret yoken'
```

## Missing and future features:

- Still can't upload files like images or pdfs.
- Some Http methods like HEAD or OPTIONS were not implemented.
- Autoinstaller.
