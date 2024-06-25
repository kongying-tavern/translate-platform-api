# 空荧酒馆 · 翻译后台

## 前期准备

```bash
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install cargo-make
cargo install wasm-bindgen-cli@0.2.92
```

## 本地调试运行

```bash
cargo make init-db
cargo make dev
```

## 上传到服务器

```bash
cargo make publish
```

> 服务器上需要安装 Nginx、Docker 与 Docker Compose，并将目录下的 `nginx.conf` 与 `reboot.sh` 上载。
>
> 需要将 `nginx.conf` 传到服务器的 `/etc/nginx/conf.d/` 下，然后在服务器上运行 `nginx -s reload` 重载 Nginx 配置。
>
> 上载到 Docker 镜像后，在服务器上运行目录下的 `reboot.sh` 即可自动完成部署，包括证书自动拉取、Nginx 刷新与 Docker 容器重启。
