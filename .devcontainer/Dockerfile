FROM rust:1

RUN useradd -m dev
USER dev

RUN rustup component add rustfmt clippy
RUN cargo install sqlx-cli