FROM alpine:3.18.2 AS alpine

RUN apk update && apk add --no-cache musl-dev libressl-dev curl gcc rust cargo

FROM alpine AS ratty

EXPOSE 8080

WORKDIR /opt/ratty/

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]

