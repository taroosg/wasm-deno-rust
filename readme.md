# DenoとRustでwasm

## ビルド

```bash
$ rustwasmc build --target deno --release
```

## 実行

```bash
$ deno run --allow-read --allow-env --unstable index.ts
```

## フォーマット

```bash
$ deno fmt
```

## Lint

```bash
$ deno lint
```
