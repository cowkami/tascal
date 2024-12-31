# ベースイメージとしてNode.jsのLTS(Long Term Support)バージョンを使用
FROM node:18-alpine

# 作業ディレクトリを/appに設定
WORKDIR /app

# アプリケーションのソースコードをコピー
COPY . .

# 依存関係をインストール
RUN npm install
RUN npm run build

# ポート番号を設定 (Viteのデフォルトは5173)
EXPOSE 5173

# コンテナ起動時に実行するコマンド
CMD ["npm", "run", "dev", "--host"]
