# OmniMem shared server image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY omnimem /usr/local/bin/omnimem
RUN chmod +x /usr/local/bin/omnimem

ENV OMNIMEM_DATA_DIR=/data
VOLUME ["/data"]
EXPOSE 8080

ENTRYPOINT ["omnimem", "http", "--bind", "0.0.0.0:8080"]