FROM lukemathwalker/cargo-chef:latest-rust-1.80.0 AS build-base
WORKDIR /app
RUN cargo install cargo-binstall
RUN cargo binstall diesel_cli -y
RUN rustup component add rust-src

FROM build-base AS planner
COPY . .
ENV PQ_LIB_STATIC="true"
RUN cargo chef prepare --recipe-path recipe.json


FROM build-base AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
ENV PQ_LIB_STATIC="true"
RUN cargo chef cook --release --bin web_api --target x86_64-unknown-linux-gnu --recipe-path recipe.json
# Build application
ENV PGSSLCERT="./db-ca-certificate.crt"
COPY . .
# database url is required at build time
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV ROCKET_DATABASES='primary={url="${DATABASE_URL}}}"'
# public base url is required at build time
ARG PUBLIC_BASE_URL
ENV PUBLIC_BASE_URL=${PUBLIC_BASE_URL}
ENV ROCKET_PUBLIC_BASE_URL=${PUBLIC_BASE_URL}

# These env entries are required for the server to start
# as 'cargo test' runs with a different working diretory than the workspace root
ENV ROCKET_TEMPLATE_DIR="/app/templates"
ENV ROCKET_PUBLIC_FILES_DIR="/app/public"

ARG ROCKET_SECRET_KEY
ENV ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY}
ENV ROCKET_PROFILE="release"
RUN cargo build --release --bin web_api --target x86_64-unknown-linux-gnu

FROM alpine as runner
ENV PGSSLCERT="/app/db-ca-certificate.crt"

# These env entries are required for the server to start
# ENV ROCKET_DATABASE_URL=
ENV ROCKET_DATABASE_URL=
ENV ROCKET_DATABASES='{primary={url="${ROCKET_DATABASE_URL}}}"'
ENV ROCKET_PUBLIC_BASE_URL=
ENV ROCKET_TEMPLATE_DIR="/app/templates"
ENV ROCKET_PUBLIC_FILES_DIR="/app/public"
ENV ROCKET_SECRET_KEY=
ENV AUTH0_CLIENT_ID=
ENV AUTH0_CLIENT_SECRET=
ENV AUTH0_DOMAIN=



WORKDIR /app
COPY --from=builder /app/db-ca-certificate.crt /app/
COPY --from=builder /app/public /app/public
COPY --from=builder /app/templates /app/templates
# COPY --from=builder /app/tls /app/tls
COPY --from=builder /app/Rocket.toml /app
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/web_api .
COPY --from=build-base /usr/local/cargo/bin/diesel .
COPY --from=builder /app/scripts/start.sh start.sh
# COPY --from=builder /app/signingkey.bin /app/


ENTRYPOINT ["/bin/sh", "/app/start.sh"]