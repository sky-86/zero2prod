FROM rust:1.67

WORKDIR /app
COPY . .

RUN cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]
