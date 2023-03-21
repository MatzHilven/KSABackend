# KSABackend

## Introduction

This repository contains the backend for the website of KSA Frisse Heikracht Bree. It is written in Rust using the
Actix Web framework. The backend is responsible for serving the website, the API and the database. It also
handles the authentication and authorization of users. The frontend is written in Next.js and can be found [here][1].
The backend also uses [Diesel][2] to communicate with the database.

[1]: https://github.com/MatzHilven/KSASite
[2]: http://diesel.rs/

## Running the backend

### Prerequisites

- [Rust][3]
- [Postgres][4]

[3]: https://www.rust-lang.org/tools/install
[4]: https://www.postgresql.org/download/

### Running the backend

To run the backend, you need to create a `.env` file in the root of the project. This file contains the
environment variables that are used by the backend. The `.env` file should look like this:

```bash
DATABASE_URL=postgres://<username>:<password>@<host>:<port>/<database>
```

You can then run the backend using the following command:

```bash
cargo run
```

## TODO

- [x] Add TLS Support & Https/2
- [ ] Add Authentication & Authorization
- [x] Add Diesel ORM & Create Database Schema using Postgres
- [ ] Add Users API
- [x] Add Activities API
- [x] Add Events API
- [ ] Add Analytics API
- [ ] Add Images API
- [ ] Add diesel cli to Dockerfile
- [ ] Add Webshop API
- [ ] Add Forms API (registrations)
