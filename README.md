# simple mini games server

:warning: WORK IN PROGRESS ⚠:warning:

This is a demo/toy project created for multiple purposes :
- learn how to make a REST API using Rust and axum
- provide API documentation for all mini games so other developers can create frontend for minigames
  
  

Game(s) currently available available :
- Guess the number ([API documentation](./guess_the_number/README.md))

## Table of contents

- [simple mini games server](#simple-mini-games-server)
  - [Table of contents](#table-of-contents)
  - [Run the server](#run-the-server)
    - [Run locally](#run-locally)
    - [Run in Docker](#run-in-docker)



## Run the server

You have 2 options :
    - run it locally on your machine
    - run it inside a Docker container

### Run locally

```sh
cargo run # optionnally add --release to build in release mode
```
> The server will listen on all interfaces, port 3000.

### Run in Docker

First, you have to build the image :
```sh
docker build -t simple_mini_games_server .
```
  
Then, run it :
```sh
docker run -d -p 3000:3000 simple_mini_games_server
```
> The exposed port is 3000, but you can change the port mapping if you want.