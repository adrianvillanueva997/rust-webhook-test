FROM rust:1.70.0-slim-bullseye AS build
WORKDIR /build
RUN apt-get update && \
    apt-get install -y apt-utils pkg-config libssl-dev --no-install-recommends && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/* /var/tmp/*
COPY . .
RUN cargo build --release

FROM alpine:3.18.2 AS prod
RUN apk update && \
    apk add --no-cache ca-certificates openssl-dev && \
    rm -rf /var/cache/apk/*
WORKDIR /app
COPY --from=build /build/target/release/webhookstuff .
RUN adduser -D -s /bin/sh appuser
USER appuser
ENV RUST_LOG=info
ENTRYPOINT [ "./webhookstuff" ]
