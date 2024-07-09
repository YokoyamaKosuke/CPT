# Builder stage
FROM rust:1-bullseye AS builder
ARG TARGETPLATFORM

WORKDIR /work/CPT
COPY . .

RUN apt-get update && apt-get install -y cmake
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim
ARG VERSION=0.5.1

LABEL org.opencontainers.image.source="https://github.com/YokoyamaKosuke/CPT" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.title="CPT" \
      org.opencontainers.image.description="CPT is a simple file transfer tool."

      RUN adduser --disabled-password --disabled-login --home /workdir nonroot \
      && mkdir -p /workdir
  
  COPY --from=builder /work/CPT/target/release/CPT /opt/CPT/CPT
  WORKDIR /workdir
  

USER nonroot
ENTRYPOINT ["/opt/CPT/CPT"]
