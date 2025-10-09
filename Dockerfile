# ----------
#    USER
# ----------
FROM alpine:latest AS user
RUN adduser -S -s /bin/false -D accord
RUN mkdir /data

###########
# Builder #
###########
FROM rust:alpine AS builder 
WORKDIR /build

# Install build dependencies
RUN apk add --update build-base cmake

# Pre-cache dependencies
COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN mkdir src \
    && echo "// Placeholder" > src/lib.rs \
    && cargo build --release \
    && rm src/lib.rs

# Build
ARG SQLX_OFFLINE true
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
COPY ["./src", "./src"]
RUN cargo build --release

###########
# Runtime #
###########
FROM scratch
COPY --from=user /etc/passwd /etc/passwd
COPY --from=user /bin/false /bin/false

USER accord
WORKDIR /opt/accord
COPY --from=user --chown=accord /data /opt/accord/data

ENV RUST_BACKTRACE=1
ENV DATABASE_URL=sqlite:///opt/accord/data/db.sqlite3
ENV DATA_PATH=/opt/accord/data
COPY --from=builder /build/target/release/accord /usr/bin/accord
ENTRYPOINT ["/usr/bin/accord"]
