# サーバーを起動
```
flutter run -d web-server --web-port=${WEB_SERVER_PORT} --web-hostname 0.0.0.0
```

# 実行中にコンパイルエラーになったとき
frontend/src直下で以下を実行
```
flutter pub get
```