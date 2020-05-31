FROM rust:1.43 as build

COPY src /opt/flatcrawl/src
COPY Cargo.toml /opt/flatcrawl
COPY Cargo.lock /opt/flatcrawl
WORKDIR /opt/flatcrawl
RUN cargo build --release

FROM debian

COPY --from=build /opt/flatcrawl/target/release/flatcrawl-crawler /opt/flatcrawl/crawler

COPY config.sample.toml /opt/flatcrawl/
RUN chmod +x /opt/flatcrawl/crawler
WORKDIR /opt/flatcrawl

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

CMD [ "./crawler" ]