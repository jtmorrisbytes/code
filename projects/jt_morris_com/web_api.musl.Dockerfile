FROM alpine
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
COPY ./db-ca-certificate.crt .
COPY ./public ./public
COPY ./templates ./templates
COPY ./target/x86_64-unknown-linux-musl/release/web_api .
COPY ./Rocket.toml .


ENTRYPOINT [ "/app/web_api" ]