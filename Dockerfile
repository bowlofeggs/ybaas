# This Dockerfile is for Dockerhub
FROM registry.fedoraproject.org/fedora:37 as builder
LABEL maintainer="Randy Barlow <randy@electronsweatshop.com>"

RUN dnf upgrade -y
RUN dnf install -y cargo
RUN mkdir /ybaas
COPY Cargo.toml /ybaas/
COPY Cargo.lock /ybaas/
COPY src /ybaas/src
RUN cd /ybaas && cargo build --release

# Build the final image
FROM registry.fedoraproject.org/fedora:37

RUN dnf upgrade -y
COPY --from=builder /ybaas/target/release/ybaas /usr/local/bin/ybaas

EXPOSE 3030/tcp
CMD ["/usr/local/bin/ybaas"]
