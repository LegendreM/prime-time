FROM rust AS build

RUN mkdir /build

WORKDIR /build

COPY . /build

RUN cargo build --release

FROM rust

COPY --from=build /build/target/release/prime-worker /

ENTRYPOINT ["/prime-worker"]