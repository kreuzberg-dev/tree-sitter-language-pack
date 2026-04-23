---
description: "Run ts-pack in Docker — a statically-linked Alpine image with all parsers compiled in."
---

# Docker

The Docker image ships a statically-linked `ts-pack` binary on Alpine Linux. All parsers are compiled in at image build time; no internet access or runtime downloads are needed.

## Quick start

```bash
docker pull ghcr.io/kreuzberg-dev/tree-sitter-language-pack:latest

# Parse a file by mounting the current directory
docker run --rm \
  -v "$(pwd):/work" -w /work \
  ghcr.io/kreuzberg-dev/tree-sitter-language-pack:latest \
  parse src/main.py

# From stdin
echo "def hello(): pass" | docker run --rm -i \
  ghcr.io/kreuzberg-dev/tree-sitter-language-pack:latest \
  parse - --language python
```

## Image contents

The image is two layers:

1. A Rust/Alpine builder that compiles `ts-pack-cli` with all parsers statically linked via `TSLP_LINK_MODE=static`
2. A minimal `alpine:latest` runtime containing only `/usr/local/bin/ts-pack`

Because the binary is statically linked against musl libc, it runs on any Linux host without additional dependencies.

## Building locally

Before building, you need the parser C sources cloned locally:

```bash
uv run scripts/clone_vendors.py
```

Then build the image from the repo root (the full context is required):

```bash
docker build -f docker/Dockerfile -t ts-pack .
```

The build takes several minutes — it compiles every grammar in `sources/language_definitions.json` from C source.

## Verify the image

```bash
docker run --rm ts-pack --version
docker run --rm ts-pack list | wc -l
```

## Use in CI

```yaml
# GitHub Actions
jobs:
  analyze:
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/kreuzberg-dev/tree-sitter-language-pack:latest
    steps:
      - uses: actions/checkout@v4
      - name: Extract structure
        run: ts-pack process src/main.py --structure
```

## Build a smaller image with a parser subset

If you only need a few languages, set `TSLP_LANGUAGES` at build time:

```dockerfile
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev gcc g++ python3 bash
WORKDIR /build
COPY . .
RUN TSLP_LANGUAGES=python,javascript,typescript \
    TSLP_LINK_MODE=static \
    PROJECT_ROOT=/build \
    cargo build --release -p ts-pack-cli && \
    strip target/release/ts-pack

FROM alpine:latest
COPY --from=builder /build/target/release/ts-pack /usr/local/bin/ts-pack
ENTRYPOINT ["ts-pack"]
```

Run `uv run scripts/clone_vendors.py --languages python,javascript,typescript` first to fetch only the needed grammar sources.

## Multi-arch

The published image is built for `linux/amd64` and `linux/arm64`. The `ci-docker.yaml` and `publish-docker.yaml` workflows handle this via `docker buildx`.
