# Build your own Docker

[![progress-banner](https://backend.codecrafters.io/progress/docker/d6f66621-6d92-4968-a1d4-fda354c07ffd)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

Welcome to the "Build Your Own HTTP Server" for Rust solutions!

## Overview

This repository serves as a my Rust solutions to the
["Build Your Own Docker" Challenge](https://codecrafters.io/challenges/docker).

You'll build a program that can pull an image from
[Docker Hub](https://hub.docker.com/) and execute commands in it. Along the way,
we'll learn about [chroot](https://en.wikipedia.org/wiki/Chroot),
[kernel namespaces](https://en.wikipedia.org/wiki/Linux_namespaces), the
[docker registry API](https://docs.docker.com/registry/spec/api/) and much more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.



## Getting started

You'll use linux-specific syscalls in this challenge. so we'll run your code
_inside_ a Docker container.

Please ensure you have [Docker installed](https://docs.docker.com/get-docker/)
locally.

Next, add a [shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mydocker='docker build -t mydocker . && docker run --cap-add="SYS_ADMIN" mydocker'
```

(The `--cap-add="SYS_ADMIN"` flag is required to create
[PID Namespaces](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html))

You can now execute your program like this:

```sh
mydocker run ubuntu:latest /usr/local/bin/docker-explorer echo hey
```

This command compiles your Rust project, so it might be slow the first time you
run it. Subsequent runs will be fast.

## Project structure
```bash
├── docker
│  ├── auth.rs
│  ├── commands.rs
│  ├── constants.rs
│  ├── image.rs
│  └── mod.rs
├── filesystem.rs
├── lib.rs
├── main.rs
└── utils.rs
```