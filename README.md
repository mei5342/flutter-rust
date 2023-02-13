# docker実行
#### ビルド：
```
docker build -t my-rust-app .
```
#### 実行：
```
docker run -it --rm -p 8081:8081 --name my-running-app my-rust-app
```

# actix-web
#### アクセス：
```
http://localhost:8081/app/index.html
```
# 参考
・wsl2 localhostアクセスエラー：https://qiita.com/snaka/items/a8eee4cfc8f7d733e6ab
