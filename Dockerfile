FROM rust:1.70 as builder
WORKDIR /server
COPY . .
RUN cargo build

FROM debian:bullseye-slim
COPY --from=builder /server/target/debug .
EXPOSE 3000
ENTRYPOINT [ "./server" ]
