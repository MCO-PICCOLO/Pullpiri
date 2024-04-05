.PHONY: default build
build: fmt
	cargo build

.PHONY: release
release: fmt
	cargo build --release

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean: fmt
	rm -rf target \
	rm -rf default.etcd \
	rm -rf bin

.PHONY: link
link: build
	$(shell ./script/link.sh)
