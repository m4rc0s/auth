FROM rust:latest AS build
 
RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 
 
RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 
 
COPY . /app
WORKDIR /app 

RUN cargo build --release


FROM rust:latest as service

COPY --from=build /app /auth-service
 
CMD ["/auth-service/target/release/auth"]