OBJ = ./target/release/libssl_write.so

run: build
	LD_PRELOAD="$(OBJ)" curl https://www.baidu.com > /dev/null
	cat log.txt
	rm log.txt

build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean