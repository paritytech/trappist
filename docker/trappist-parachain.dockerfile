# This file is sourced from https://github.com/paritytech/polkadot/blob/master/scripts/ci/dockerfiles/polkadot/polkadot_builder.Dockerfile
FROM docker.io/paritytech/ci-linux:1.68.2-bullseye as builder

WORKDIR /trappist
COPY . /trappist

RUN cargo build --release

# the collator stage is normally built once, cached, and then ignored, but can
# be specified with the --target build flag. This adds some extra tooling to the
# image, which is required for a launcher script. The script simply adds two
# arguments to the list passed in:
#
#   --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID
#
# with the appropriate ip and ID for both Alice and Bob
FROM debian:bullseye-slim as collator
RUN apt-get update && apt-get install jq curl bash -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh && \
    curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
    apt-get install -y nodejs && \
    npm install --global yarn && \
    yarn global add @polkadot/api-cli@0.10.0-beta.14
COPY --from=builder \
    /trappist/target/release/trappist-node /usr/bin
COPY ./docker/scripts/inject_bootnodes.sh /usr/bin
CMD ["/usr/bin/inject_bootnodes.sh"]
COPY ./docker/scripts/healthcheck.sh /usr/bin/
HEALTHCHECK --interval=300s --timeout=75s --start-period=30s --retries=3 \
    CMD ["/usr/bin/healthcheck.sh"]

# the runtime stage is normally built once, cached, and ignored, but can be
# specified with the --target build flag. This just preserves one of the builder's
# outputs, which can then be moved into a volume at runtime
FROM debian:bullseye-slim as runtime
COPY --from=builder \
    /trappist/target/release/wbuild/trappist-runtime/trappist_runtime.wasm \
    /var/opt/
CMD ["cp", "-v", "/var/opt/trappist_runtime.wasm", "/runtime/"]

FROM debian:bullseye-slim
COPY --from=builder \
    /trappist/target/release/trappist-node /usr/bin

CMD ["/usr/bin/trappist-node"]
