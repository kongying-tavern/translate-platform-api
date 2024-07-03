# Build SQL
FROM node:lts AS builder

WORKDIR /data
ADD docker/builder/dataenv .
COPY database/pdmaner/空荧翻译平台.pdma.json data.pdma.json

RUN corepack enable && \
    cd /data && \
    rm -rf node_modules && \
    pnpm i && \
    pnpm build -s ./data.pdma.json -t ./database.sql

# Create Runner
FROM postgres:15 AS runner
COPY --from=builder /data/database.sql /docker-entrypoint-initdb.d/initialize-db.sql
