FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/ksabackend
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/src/ksabackend/target/release/backend /usr/local/bin/backend

EXPOSE 80

WORKDIR /usr/local/bin
CMD ["backend"]