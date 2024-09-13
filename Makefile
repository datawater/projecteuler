THREADS = $(shell nproc --all)
LINKER = $(shell if command -v mold   &> /dev/null; then echo \"-C link-arg=-fuse-ld=mold\"; \
			   elif command -v ld.lld &> /dev/null; then echo \"-C link-arg=-fuse-ld=lld\"; fi)
RUSTCFLAGS = "-Clink-args=-Wl,--build-id -C target-cpu=native "$(LINKER)

all: debug

about:
	cargo about init
	cargo about generate about.hbs > license.html

fix:
	cargo fix --allow-dirty

release:
	RUSTFLAGS=$(RUSTCFLAGS) cargo build --release -j$(THREADS) -v

debug:
	RUSTFLAGS=$(RUSTCFLAGS) cargo build --profile=dev -j$(THREADS) -v

clean:
	cargo clean	