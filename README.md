# nes emulator

## Build Docker Image

```bash
cd ./docker/based-on-debian
docker build -t cross-compilation .
cd ..
mkdir -p $PWD/.docker/.cargo/git
mkdir -p $PWD/.docker/.cargo/registry
```

## Ubuntu amd64

```bash
docker run --rm -it \
        -v $PWD:/build \
        -v $PWD/.docker/.cargo/registry:/root/.cargo/registry \
        -v $PWD/.docker/.cargo/git:/root/.cargo/git \
        cross-compilation /amd64-debug
```

## Raspberry Pi

```bash
docker run --rm -it \
        -v $PWD:/build \
        -v $PWD/.docker/.cargo/registry:/root/.cargo/registry \
        -v $PWD/.docker/.cargo/git:/root/.cargo/git \
        cross-compilation /aarch64-release
```
