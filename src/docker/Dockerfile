# ベースイメージとしてRustの公式イメージを使用
FROM rust:latest

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# ソースコードをコンテナにコピー
COPY . .

# Rustアプリケーションをビルド
RUN cargo build --release

# ポートを公開
EXPOSE 8080

# アプリケーションを実行
CMD ["./target/release/your-app-name"];