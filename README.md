# Grapple

## A Github Mirroring Tool

Grapple is a tool for mirroring a GitHub based repository by listening for webhooks, pulling changes, and pushing them to a remote repository. Functionality is fairly basic right now, but it can listen to multiple repositories and mirror them. The webserver is built with [Rust](https://www.rust-lang.org/) and the [Rocket](https://rocket.rs) web framework.

## Installing

Grapple relies on features in nightly builds. Using [rustup](https://www.rustup.rs/), use a recent nightly build, e.g.:

```
rustup default nightly-2017-08-22
```

And then build with `cargo`:

```
cargo build --release
```

This will produce the executable in `target/release/grapple`. The Dockerfile in `./docker` can help you spin it up as a webserver if you copy the release binary into the docker folder.

## Docker image/running Grapple

A recent docker image is hosted at this registry address:
`registry.gitlab.frielforreal.com/aelve/grapple`. Run with:

```
ROCKET_CONFIG_PATH="/var/lib/grapple"
ROCKET_PORT="80"

docker run --name grapple --restart=unless-stopped \
    -v ${ROCKET_CONFIG_PATH}:/var/lib/grapple \
    -e ROCKET_ENV=stage \
    -e ROCKET_PORT=80 \
    -p ${ROCKET_PORT}:80/tcp \
    -d registry.gitlab.frielforreal.com/aelve/grapple grapple
``

The server will listen at `0.0.0.0/github` for webhooks. Verify the server is running with `docker logs`.

## Configuration

Grapple supports `yaml` and `json` configuration formats. A sample file would look like:

```
{
    "mappings": [
        {
            "from": "aelve/guide",
            "push_uri": "ssh://git@gitlab.frielforreal.com:58432/aelve/guide.git",
            "deploy_public_key": "/var/lib/grapple/aelve_rsa.pub",
            "deploy_private_key": "/var/lib/grapple/aelve_rsa",
            "secret": "SECRET HERE"
        }
    ]
}
```

This config file listens for GitHub webhooks from the repository `aelve/guide`, and pushes them to my gitlab server. The SSH keys should be deploy keys or otherwise access-limited keys that can only push to the repository of your choice. (Not that I think Grapple is insecure, but these are best practices.) The secret should be the GitHub WebHook secret, which is used to validate the incoming requests.

## What if something breaks?

This tool largely builds on existing Rust infrastructure, including the wonderful [git2](https://github.com/alexcrichton/git2-rs) by Alex Crichton. There are almost certainly edge cases that I haven't thought about (e.g.: what happens if someone force pushes to GitHub? I don't know!). If you have any feedback for how to improve it, please submit an issue or pull request for any tests.