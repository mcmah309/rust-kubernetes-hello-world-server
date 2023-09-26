FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/hello_world_kubernetes ./

RUN chmod +x hello_world_kubernetes

CMD ["./hello_world_kubernetes"]