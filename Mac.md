# Step 1

Ensure that macOS and XCode are up to date

# Step 2

Install homebrew if you haven’t already.

    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Step 3

Install PostgreSQL

    brew install postgresql
    brew services start postgresql
    createdb

# Step 4

Install PostGIS

    brew install postgis

# Step 5

Install yarn

    brew install yarn

# Step 6

Install Rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

At the end of that process, it will instruct you to either close your
terminal and open a new one, or source a file. Do one of those things.

# Step 7

Install SQLX command line tool

    cargo install sqlx-cli

# Step 8

Clone the code repository

    git clone git @github.com:SciStarter/Circuit.git

That command assumes that you have write access to the repository, and
your SSH public key on file with Github. You can alternatively use the
HTTPS protocol clone to get read-only access to the repository, in
which case you’ll need to submit your changes as pull requests.

# Step 9

Set environment variables. These instructions are for using an env
file, but any method of setting and managing environment variables
will work.

To use an env file, first create a text file called **circuit.env**
in your home directory containing the following:

    export SUPERUSER_EMAIL=[some email of your choice]
    export SUPERUSER_PASSWORD=[password of your choice]
    export INTERNAL_UID=69e9ccb8-cd72-412b-a291-9158a72f84c7
    export JWT_SIGNING_KEY=not-a-great-key
    export DATABASE_URL=postgres://127.0.0.1/
    export DOMAIN=localhost
    export LOCAL_API_URL=http://localhost:8000

Save that. Now, any time you want to work on Science Near Me,
including running a local server, open a terminal and run

    source circuit.env

After that, you’ll be able to use that terminal to run the frontend
server or the api server.

The rest of the steps assume that you’re working in a terminal where
you’ve sourced this file or otherwise set up the environment
variables.

There are two other environment variables that you might need, but
they are secrets. Contact Daniel if you find you need OPENCAGE_API_KEY
or MAPBOX_TOKEN

# Step 10

Install javascript dependencies (the cd command assumes you’re
starting from the place where you ran the git clone command)

    cd Circuit/frontend
    yarn install

# Step 11

Set up the database schema (the cd command assumes you’re starting
from the place where you ran the git clone command)

    cd Circuit/common
    sqlx migrate run

# Step 12

Import some data for testing (the cd command assumes you’re starting
from the place where you ran the git clone command)

    cd Circuit/importer
    cargo run

It will take a little while to compile, the first time you run it.
Then it will tell you it’s importing and provide some details. It may
report some errors due to badly formatted data items, then it will
print out a line starting with “Imported.” At that point, you can
press Ctrl-C to stop the importer; it will keep running forever on its
own and you don’t need it anymore.

# Step 13

Run the API server. This is where you start if you want to run the
server again in the future, as well.

First, make sure you have the environment variables set, as described
above. Then, in the directory where you checked out the git
repository, do:

    cd Circuit/api
    cargo run

This will run the API server (after spending a little while compiling,
the first time). Leave this terminal open until you’re ready to shut
down your local server.

# Step 14

Run the frontend server. In a new terminal window (not the one you
used to run the API server in the previous step), first, make sure you
have the environment variables set up, then, in the directory where
you checked out the git repository, do:

    cd Circuit/frontend
    yarn dev -o

# Done

Your local server is up and running on http://localhost:3000/ and
should have opened the site in your browser automatically.

You can access the admin area on http://localhost:3000/api/v1/manage/
using the superuser email and password that you set in the environment
variables.

The dev server supports HMR, so as you make changes in the frontend
files, they will be automatically rebuilt and your open browsers will
be updated.
