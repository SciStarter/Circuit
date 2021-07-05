# frontend

## Running a full local instance

There are three parts to a local instance. One part is the Nuxt server
contained in this folder, which renders the front-end of the site. The
other two parts are a PostgreSQL database, and an API server which is
contained in the ../api folder.

To run a full local instance, we need to:

1. set up and run PostgreSQL

2. set up and run the API server

3. set the LOCAL_API_URL environment variable to http://127.0.0.1:8000

4. run the Nuxt server using ```yarn dev```

### PostgreSQL database

The full details about how to set up a local PostgreSQL database are
beyond the scope of this readme, and are well documented elsewhere.
The short version is this:

* Install [PostgreSQL](https://www.postgresql.org/download/) and
  [PostGIS](https://postgis.net/install/) using whatever mechanism
  you’re comfortable with. Linux package managers and Docker are both
  convenient options.

* Make sure the credentials are set up so that a local process will
  have read/write access to an empty database on that PostgreSQL
  server.

* Make sure the PostgreSQL server is running.

### API server

The API server requires a local installation of
[Rust](https://www.rust-lang.org/), which can be easily installed
using [rustup](https://rustup.rs/) or various package managers.

Once Rust is installed, you should have access to a command line
program called Cargo. You can check by running:

    cargo --version

If cargo works, we need to set some environment variables:

variable           |value
-------------------|-----
DATABASE_URL       | Set this to the database URL which will allow the API server to access the PostgreSQL database. For example: ```DATABASE_URL=postgres://postgres@127.0.0.1/circuit-api```
JWT_SIGNING_KEY    | Set this to some random value. For example: ```JWT_SIGNING_KEY=example-key```
SUPERUSER_EMAIL    | Set this to the email address you want to use for accessing the [management area](http://127.0.0.1:8000/api/v1/manage/) of your local install.
SUPERUSER_PASSWORD | Set this to the password you want to use for accessing the management area.

Now we can run the server by doing:

    cargo run

## Build Setup

```bash
# install dependencies
$ yarn install

# serve with hot reload at localhost:3000
$ yarn dev

# build for production and launch server
$ yarn build
$ yarn start

# generate static project
$ yarn generate
```

For detailed explanation on how things work, check out [Nuxt.js docs](https://nuxtjs.org).
