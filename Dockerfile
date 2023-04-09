FROM alpine AS builder

RUN apk add --no-cache cargo openssl-dev

RUN mkdir /src
WORKDIR /src
COPY . .
RUN cargo build --release --locked



FROM alpine

RUN apk add --no-cache libgcc libssl3
COPY --from=builder /src/target/release/synapse_compressor /usr/local/bin/synapse_compressor

CMD ["/usr/local/bin/synapse_compressor"]
