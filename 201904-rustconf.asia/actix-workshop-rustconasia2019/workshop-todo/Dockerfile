# stage 0
FROM clux/muslrust:stable as builder

COPY ./crate_config /root/.cargo/config

# RUN cargo install diesel_cli --no-default-features --features postgres

ENV APP_ROOT=/var/www/workshop
RUN mkdir -p $APP_ROOT

WORKDIR $APP_ROOT
COPY Cargo.toml Cargo.lock diesel.toml $APP_ROOT/

COPY db $APP_ROOT/src
# Build our application.
RUN cargo build --release --target x86_64-unknown-linux-musl

# stage 1
# Now, we need to build our _real_ Docker container, copying in `app`.
FROM gliderlabs/alpine:3.7

# COPY --from=builder /root/.cargo/bin/diesel /usr/local/bin/
# CMD /usr/local/bin/diesel

# change China Mirror
RUN echo http://mirror.yandex.ru/mirrors/alpine/v3.5/main > /etc/apk/repositories; \
    echo http://mirror.yandex.ru/mirrors/alpine/v3.5/community >> /etc/apk/repositories

RUN apk update && apk --no-cache add ca-certificates

COPY --from=builder \
    /var/www/workshop/target/x86_64-unknown-linux-musl/release/workshop-todo \
    /usr/local/bin/

CMD /usr/local/bin/workshop-todo
EXPOSE 8080
