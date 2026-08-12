# NOTE: The `parsers/` directory must exist in the build context.

FROM rust:1.95-alpine3.22 AS builder

SHELL ["/bin/ash", "-eo", "pipefail", "-c"]

RUN apk add --no-cache \
    bash=5.2.37-r0 \
    g++=14.2.0-r6 \
    gcc=14.2.0-r6 \
    musl-dev=1.2.5-r12 \
    python3=3.12.13-r0

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=gcc

WORKDIR /build
COPY . .

RUN LANGS=$(python3 -c "import json; d=json.load(open('sources/language_definitions.json')); print(','.join(sorted(d.keys())))") && \
    echo "Building with $(echo "$LANGS" | tr ',' '\n' | wc -l) languages" && \
    TSLP_LANGUAGES="$LANGS" \
    TSLP_LINK_MODE=static \
    PROJECT_ROOT=/build \
    cargo build --release -p ts-pack-cli && \
    strip target/release/ts-pack

FROM alpine:3.22

SHELL ["/bin/ash", "-eo", "pipefail", "-c"]

COPY --from=builder /build/target/release/ts-pack /usr/local/bin/ts-pack

# ~keep All grammars are linked statically (TSLP_LINK_MODE=static above), so the parser
# ~keep cache is only used by `download`/`clean`. The env var sets the cache *base*
# ~keep (`<base>/tree-sitter-language-pack/v{version}/libs`); pin it inside the non-root
# ~keep user's home so it stays writable regardless of how HOME is resolved.
ENV HOME=/home/tspack \
    TREE_SITTER_LANGUAGE_PACK_CACHE_DIR=/home/tspack/.cache

RUN addgroup -S -g 65532 tspack && \
    adduser -S -D -u 65532 -G tspack -h /home/tspack -s /sbin/nologin tspack && \
    mkdir -p "$TREE_SITTER_LANGUAGE_PACK_CACHE_DIR" && \
    chown -R tspack:tspack /home/tspack

USER tspack:tspack

RUN ts-pack --version && \
    echo "def hello(): pass" | ts-pack parse - --language python

ENTRYPOINT ["ts-pack"]
