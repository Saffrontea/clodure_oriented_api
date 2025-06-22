# ベースイメージ
FROM rust:1.87.0-bullseye as builder

WORKDIR /app

# 必要なパッケージのインストール（MySQLクライアント含む）
RUN apt-get update && \
    apt-get install -y default-libmysqlclient-dev pkg-config build-essential libssl1.1 openssl libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# ソースコードのコピー
COPY . .

# リリースビルド
RUN cargo build --release

# ランタイム用イメージ
FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update && \
    apt-get install -y default-libmysqlclient-dev libssl1.1 openssl libssl-dev procps && \
    rm -rf /var/lib/apt/lists/*
RUN mkdir -p /app/log
VOLUME [ "/app/log" ]

COPY --from=builder /app/target/release/clodure_oriented_api /app/
COPY --from=builder /app/memory_monitoring_and_load_testing.sh /app/

# COPY --from=builder /app/.env /app/.env

CMD ["/app/memory_monitoring_and_load_testing.sh"]
