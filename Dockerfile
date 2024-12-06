FROM rust:1.82.0-alpine3.20 AS build

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache build-base libgcc musl-dev libressl pkgconfig libpq-dev curl

WORKDIR /app

COPY . .

RUN cargo build --release

# production environment
FROM rust:1.82.0-alpine3.20

RUN apk add --no-cache libgcc libpq-dev ca-certificates curl

WORKDIR /app

COPY --from=build /app/files ./files
COPY --from=build /app/target/release/ghelere-ai .

RUN mkdir -p ./src/upload

EXPOSE 8080

CMD [ "./ghelere-ai" ]
