# 具体镜像请联系 langyo.china@gmail.com 授予权限获取

# 第一次部署前先执行 docker 登录指令
# docker login --username=langyo_china registry.cn-shanghai.aliyuncs.com

# certbot certonly --webroot -w /usr/share/nginx/html -d kongying-tavern -m langyo.china@gmail.com --agree-tos
certbot renew --quiet
nginx -s reload

docker pull registry.cn-shanghai.aliyuncs.com/langyo/kongying-tavern-boot:latest

docker compose -f Compose.yml up -d
