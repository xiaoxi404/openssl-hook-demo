BUILD_DIR = ./build
OBJS = libssl_write.so libhook.so
BUILD_OBJS = $(OBJS:%.so=$(BUILD_DIR)/%.so)
CFLAGS = -fPIC -lssl -shared -D_GNU_SOURCE -Wall
CC = cc

run: build
	LD_PRELOAD="$(BUILD_OBJS)" curl https://www.baidu.com > /dev/null
	cat log.txt
	rm log.txt

build: $(BUILD_OBJS)

$(BUILD_DIR)/libssl_write.so: ssl_write/src/lib.rs
	mkdir -p $(BUILD_DIR)
	cd ssl_write; cargo build --release
	cp ssl_write/target/release/*.so $(BUILD_DIR)/

$(BUILD_DIR)/libhook.so: hook.c
	$(CC) $(CFLAGS) $? -o $@

.PHONY: clean
clean:
	rm -r $(BUILD_DIR)