# MCWR
### THIS DOES NOT WORK YET, DO NOT USE IT
MCWR is a minecraft server wrapper written in rust. It aims to have as simple and concise syntax as possible while still being reasonably feature complete.

**At the moment the only feature is downloading paper server jars, I'm not really sure how to continue with this so I'll leave in this state until I can figure out solutions for running the server as a child.

## Installation

#### Requirements
- [Rust](https://rustup.rs/)
- Java
- tmux (`sudo apt install tmux`)

Run the following command to install MCWR on your system. Make sure `~/.cargo/bin/` is in your PATH.

    cargo install https://github.com/DaCaseo/mcwr
## Features
- Run servers in background and access console
- Quick and easy creation of vanilla, paper/spigot, bungeecord, and modded servers
- Quick shortcuts for common actions like editing config files and restarting the server
- Simple backup solution

## Goals
- Commands should be simple to understand and as short as possible
- Few unnecessary features (no need to do in a wrapper what plugins can already do)

## Disclaimers
I wrote this primarily to learn rust and help me manage my own servers. I would love feedback and suggestions but since I am new to rust ***EXPECT ISSUES***
