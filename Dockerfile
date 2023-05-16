FROM ghcr.io/railwayapp/nixpacks:ubuntu-1675123887

ENTRYPOINT ["/bin/bash", "-l", "-c"]
WORKDIR /app/


COPY .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix
RUN nix-env -if .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix && nix-collect-garbage -d


ARG NIXPACKS_METADATA ROCKET_ADDRESS
ENV NIXPACKS_METADATA=$NIXPACKS_METADATA ROCKET_ADDRESS=$ROCKET_ADDRESS

# setup phase
# noop

# build phase
COPY . /app/.
RUN --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=LbwHGvgJb5U-target,target=/app/target mkdir -p bin
RUN --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=LbwHGvgJb5U-target,target=/app/target cargo build --release --target x86_64-unknown-linux-musl
RUN --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=LbwHGvgJb5U-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=LbwHGvgJb5U-target,target=/app/target cp target/x86_64-unknown-linux-musl/release/rust-serve bin


# start
FROM ubuntu:jammy
WORKDIR /app/
COPY --from=0 /etc/ssl/certs /etc/ssl/certs
RUN true
EXPOSE 8080
COPY --from=0 /app/bin/rust-serve /app/bin/rust-serve
CMD ["./bin/rust-serve"]

