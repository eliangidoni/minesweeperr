FROM rust:1.78
LABEL maintainer="Elian Gidoni <elianmdp@gmail.com>"
RUN mkdir /project
WORKDIR /project
ADD . /project/
RUN cargo build --release