FROM alpine:3.18.2 AS alpine

RUN apk update && apk add --no-cache musl-dev libressl-dev curl gcc rust cargo

FROM alpine AS ratty-build

EXPOSE 8080

WORKDIR /opt/ratty/

COPY . .

RUN cargo build --release

FROM alpine:3.18.2 AS ratty-prod

WORKDIR /opt/ratty/

COPY --from=ratty-build /opt/ratty/target/release/presentation .

CMD ["./presentation"]

