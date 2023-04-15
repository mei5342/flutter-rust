# docker実行
#### 実行
```
docker compose up
```
- app1: api
- app2: app1呼び出し用

# actix-web
#### アクセス：
- app2を立ち上げると、main関数でgetメソッドが実行される
# 参考
- [wsl2 localhostアクセスエラー](https://qiita.com/snaka/items/a8eee4cfc8f7d733e6ab)
- [flutter環境構築メモ](https://qiita.com/__yuporon__/items/f06a6361dbc395bf75a9)
- [mysql環境構築](https://zenn.dev/ryo7/articles/create-mysql-on-docker)