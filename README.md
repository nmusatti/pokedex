Pokedex - A simple Pokemon retrieval RESTful service
----------------------------------------------------

Pokedex is a minimal REST API implemented in Rust that showcases
[Rocket](https://github.com/SergioBenitez/Rocket) as web framework and
[Reqwest](https://github.com/seanmonstar/reqwest) as web client library. It provides two endpoints:

- `/pokemon/<<Name>>`: returns information on the `<<Name>>` pokemon
- `/pokemon/translated/<<Name>>`: returns the same information, but with the description translated
  in a funny way

In order to build and run the project on a your computer you need a Rust toolchain. To install one
on a Linux workstation you should run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Checkout the [official documentation](https://forge.rust-lang.org/infra/other-installation-methods.html)
if you use a different system. To update your shell's environment so as to be able to run the
`cargo` command you should run:

```bash
source $HOME/.cargo/env
```

Now, in the directory where you cloned the Pokedex repository you can run:

```bash
cargo run
```

The first time around it will take a while, because `cargo` has to download and build all of the
project's dependencies. At the end you should see output similar to:

```
  Configured for debug.
   >> address: 0.0.0.0
   >> port: 8000
   >> workers: 8
   >> ident: Rocket
   >> keep-alive: 5s
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> tls: disabled
   >> temp dir: /tmp
   >> log level: normal
   >> cli colors: true
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   Routes:
   >> (plain) GET /pokemon/<name>
   >> (translated) GET /pokemon/translated/<name>
  Catchers:
   >> (not_found) 404
  Fairings:
   >> Shield (liftoff, response, singleton)
  Shield:
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
  Rocket has launched from http://0.0.0.0:8000
```

At this point you can open your browser and enter:

```
http://localhost:8000/pokemon/butterfree
```

You should see output similar to:

```json
{
  "name":"butterfree",
  "description":"In battle, it\nflaps its wings\nat high speed to\frelease highly\ntoxic dust into\nthe air.",
  "habitat":"forest",
  "is_legendary":false
}
```

Pokedex may also be run as a Docker container. If the `docker` command is not available on your
system, check your system documentation or the
[official installation guide](https://docs.docker.com/get-docker/). Once you're done you can build
Pokedex's container image:

```bash
docker build -t pokedex .
```

And then execute it:

```bash
docker run -p 8000:8000 -d pokedex
```

You should see the container's id almost immediately, which is a long string of hexadecimal digits.
When it appears you can query the service with your browser as described above.

If you are interested in Kubernetes, check out [how to deploy Pokedex to Kubernetes](kubernetes)
