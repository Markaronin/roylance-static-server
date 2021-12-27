set -xe

TARGET_ARCH=armv7-unknown-linux-gnueabihf

cargo build --release --target=$TARGET_ARCH

docker build \
    --tag roylance-static-server \
    --build-arg TARGET_ARCH=$TARGET_ARCH \
    .
docker save roylance-static-server:latest | gzip | ssh pi@10.0.0.186 sudo docker load