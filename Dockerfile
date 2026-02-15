FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY codegang-datasource /usr/local/bin/codegang-datasource

EXPOSE 8080

CMD ["codegang-datasource"]
