# lambda-hello-rust

```shell
brew install filosottile/musl-cross/musl-cross     
rustup target add x86_64-unknown-linux-musl 
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

```shell
sam build
sam deploy
```
