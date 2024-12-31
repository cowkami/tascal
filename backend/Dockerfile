# ビルドステージ
FROM rust:1.83.0-slim AS builder

WORKDIR /app

# ソースコードのコピーとビルド
COPY . .
RUN cargo build --release

# ランタイムステージ
FROM debian:stable-slim

# ビルドステージからバイナリをコピー
COPY --from=builder /app/target/release/server /app/server

# 実行コマンド
CMD ["/app/server"]
