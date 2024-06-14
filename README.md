# nes emulator

## Raspberry Pi

```bash
cd rust-sdl2-aarch64
docker build -t rust-sdl2-aarch64 .
cd ..
mkdir -p $PWD/.docker/.cargo/git
mkdir -p $PWD/.docker/.cargo/registry
docker run --rm -it \
        -v $PWD:/build \
        -v $PWD/.docker/.cargo/registry:/root/.cargo/registry \
        -v $PWD/.docker/.cargo/git:/root/.cargo/git \
        rust-sdl2-aarch64 /release
```
