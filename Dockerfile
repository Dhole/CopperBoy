FROM ubuntu:20.04

RUN apt-get update && apt-get install -y curl
RUN apt-get install -y build-essential
RUN apt-get install -y gcc-aarch64-linux-gnu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu

RUN mkdir -p /src
WORKDIR /src
