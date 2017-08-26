FROM liuchong/rustup:nightly

WORKDIR /build
ADD . /build

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates zlib1g-dev libc6 libgcc1 cmake

RUN cargo build --release

# # Production build:
FROM ubuntu

WORKDIR /var/lib/grapple
ENV ROCKET_ENV=prod
ENV ROCKET_PORT=8000

COPY --from=0 /build/target/release/grapple /bin/grapple

CMD /bin/grapple
