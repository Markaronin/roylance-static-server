FROM debian:buster-slim
ARG TARGET_ARCH

COPY ./target/$TARGET_ARCH/release/roylance-static-server .

ENTRYPOINT ["./roylance-static-server"]