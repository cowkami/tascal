ARG APP_NAME=tascal

# ビルドステージ
FROM rust:1.83.0-slim AS builder
ARG APP_NAME

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
cp ./target/release/$APP_NAME /usr/local/bin/$APP_NAME
EOF

# ランタイムステージ
FROM debian:stable-slim
ARG APP_NAME

# ビルドステージからバイナリをコピー
COPY --from=builder /usr/local/bin/$APP_NAME /usr/local/bin/$APP_NAME

# 実行コマンド
CMD $APP_NAME
