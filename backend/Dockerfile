# ビルドステージ
FROM rust:1.83.0-slim AS builder

WORKDIR /work

# ソースコードのコピーとビルド
COPY . .
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/work/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/tascal /usr/local/bin/tascal
EOF

# ランタイムステージ
FROM debian:stable-slim

# ビルドステージからバイナリをコピー
COPY --from=builder /usr/local/bin/tascal /usr/local/bin/tascal

# 実行コマンド
CMD tascal
