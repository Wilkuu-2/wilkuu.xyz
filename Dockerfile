FROM rust:1.72

WORKDIR /app

# Optimize fetching cargo index files
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse 

RUN  --mount=type=cache,target=/usr/local/cargo/registry \
    sh -c "cargo install cargo-watch"

