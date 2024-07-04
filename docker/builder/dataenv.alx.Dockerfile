FROM alpine:latest

WORKDIR /data
COPY docker/builder/dataenv.alx .

VOLUME ["/data/database"]
ENTRYPOINT ["/bin/sh", "/data/reset-db.sh"]
