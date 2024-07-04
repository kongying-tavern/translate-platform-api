# Build SQL
FROM node:lts AS builder

WORKDIR /data
ADD docker/builder/dataenv/builder .
COPY database/pdmaner/空荧翻译平台.pdma.json data.pdma.json

RUN corepack enable && \
    cd /data && \
    rm -rf node_modules && \
    pnpm i && \
    pnpm build -s ./data.pdma.json -t ./database.sql

# Create Runner
FROM postgres:15 AS runner

WORKDIR /docker-entrypoint-initdb.d
COPY --from=builder /data/database.sql 000_init-db.sql
ADD docker/builder/dataenv/sql .
