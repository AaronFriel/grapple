FROM ubuntu

WORKDIR /var/lib/grapple

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl1.0.0 zlib1g libc6 libgcc1

ENV ROCKET_ENV=prod
ENV ROCKET_PORT=8000

ADD grapple /bin/grapple

CMD /bin/grapple
