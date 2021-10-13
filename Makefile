UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
OPEN_COMMAND := "xdg-open"
else
OPEN_COMMAND := "open"
endif

test:
	RUSTFLAGS="-C target-cpu=native" cargo test --features fuzz --release -- --nocapture

bench:
	RUSTFLAGS="-C target-cpu=native" cargo bench --features fuzz -- --nocapture