FROM rust:slim AS builder
SHELL ["/bin/bash", "-uo", "pipefail", "-c"]

ENV TARGET x86_64-unknown-linux-musl
RUN rustup target add "$TARGET"

RUN mkdir /src
WORKDIR /src
COPY . .
RUN cargo build --release --locked --target "$TARGET" \
 && mv "target/$TARGET/release/synapse_compressor" .



FROM scratch

COPY --from=builder /src/synapse_compressor /bin/synapse_compressor

CMD ["/bin/synapse_compressor"]
