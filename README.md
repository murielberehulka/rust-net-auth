# RUST NET AUTH

![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)
![APIs](https://img.shields.io/badge/Rust-gray?logo=rust&style=flat-square)

rust_net_auth is an open source plugin for [rust_net api](https://github.com/murielberehulka/rust_net)

It aims to be an fast, safe and easy-to-use api.

## Goal

The Goal is to make an api capable of handle clients with multiple users stored using an [mongodb](https://www.mongodb.com) database.

## Routes

Post routes list:

<details>
<summary>client/exists/by/email</summary>
If an user with this email exists

- Request: "email"
- Return:
    - 200: "Found"
    - 400: "Not found"
    - 500: server internal error message
</details>

<details>
<summary>client/exists/by/token</summary>
If an user with this token exists

- Request: "token"
- Return:
    - 200: "Found"
    - 400: "Not found"
    - 500: server internal error message
</details>

<details>
<summary>client/new</summary>
Create new client

- Request: "email|password"<br>
    email must be longer than 5 and shorter than 40<br>
    password must be longer than 5 and shorter than 40
- Return:
    - 200: client token
    - 400:
        - "Bad request format"
        - "E-mail is too short"
        - "E-mail is too long"
        - "Password is too short"
        - "Password is too long"
        - "E-mail is already in use"
    - 500: server internal error message
</details>

<details>
<summary>client/login</summary>
Login and get token

- Request: "email|password"
- Return:
    - 200: client token
    - 400:
        - "Bad request format"
        - "E-mail not found"
        - "Wrong password
    - 500: server internal error message
</details>


<details>
<summary>client/user</summary>
Get user info

- Request: "user_token"<br>
- Return:
    - 200: name|prof_pic|has_password|privilege
    - 400:
        - "User not found"
    - 500: server internal error message
</details>

<details>
<summary>client/users</summary>
Get client users list

- Request: client token
- Return:
    - 200: "u1_name|u1_prof_pic|u1_has_password|u1_privilege#u2_name|u2_prof_pic..."<br>
        users are splited by: '#'<br>
        user properties are splited by: '|'<br>
        properties:<br>
            - name<br>
            - profile picture url<br>
            - if user has password ("true" or "false")
            - privilege (admin level)
    - 400:
        - "Bad request format"
        - "Client not found": means that wrong token was given
    - 500: server internal error message
</details>

<details>
<summary>client/users/new</summary>
Create new user in client

- If there is no other user with the highest admin level, this will take it, otherwise will have 0 privilege
- Request: "client_token|user_name|user_password"<br>
    leave user_password empty for no password<br>
    user_name must be longer than 5 and shorter than 40<br>
    user_password must shorter than 40
- Return:
    - 200: "OK"
    - 400:
        - "Bad request format"
        - "Name is too short"
        - "Name is too long"
        - "Password is too long"
        - "Client not found": means that wrong token was given
        - "Name already in use"
    - 500: server internal error message
</details>

<details>
<summary>client/users/login</summary>
Login into user and get user token

- Request: "client_token|user_name|user_password"<br>
- Return:
    - 200: user token
    - 400:
        - "Bad request format"
        - "Client not found": means that wrong token was given
        - "User not found": means that wrong user_name was given
        - "Wrong password"
    - 500: server internal error message
</details>

## Todo

These are the up coming features:
- [x] Password encryption
- [x] Multiple users inside client
- [ ] Upload user profile picture
- [ ] Change user name

## 🚀 Running examples
```
cargo run --example <example-name>
```

choose one of the examples:
- [basic](https://github.com/murielberehulka/rust_net_auth/blob/master/examples/basic.rs)

## License

Licensed under of
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

Fell free to use the code as you want.