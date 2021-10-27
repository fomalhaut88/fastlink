FROM rustlang/rust:nightly-slim

WORKDIR /app
EXPOSE 5000

COPY . .

RUN cargo build --release

ENV FASTLINK_HOST=0.0.0.0
ENV FASTLINK_PORT=5000
ENV DB_DATA_PATH=db/fastlink.data
ENV DB_STATE_PATH=db/fastlink.state

CMD ./target/release/fastlink
