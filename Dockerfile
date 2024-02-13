FROM rust:1.67

WORKDIR /harsha
COPY . .

RUN cargo install --path .

CMD ["harsha_bin"]