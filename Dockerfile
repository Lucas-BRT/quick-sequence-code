FROM rust:slim AS dependency-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY qsc-generator/Cargo.toml qsc-generator/
COPY server/Cargo.toml server/

RUN mkdir -p qsc-generator/src server/src && \
    echo "fn main() {}" > qsc-generator/src/lib.rs && \
    echo "fn main() {}" > server/src/main.rs

RUN cargo build --release

FROM rust:slim AS codebase-builder

WORKDIR /app

COPY --from=dependency-builder /app/target /target
COPY server/ server/
COPY qsc-generator/ qsc-generator/
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM rust:slim AS runner

RUN cargo install wasm-pack

WORKDIR /app

COPY --from=codebase-builder /app/ .
COPY run.sh app/run.sh
COPY public /app/public

RUN wasm-pack build ./qsc-generator \
    --target web \
    --out-dir ../pkg \
    --release

ENTRYPOINT [ "cargo", "run", "--release" ]
