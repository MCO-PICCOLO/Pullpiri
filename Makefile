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
	rm -rf target/ && rm -rf default.etcd/

.PHONY: d
d:
	cargo run --bin piccolod

.PHONY: ctl
ctl:
	cargo run --bin piccoloctl $(filter-out $@,$(MAKECMDGOALS))

.PHONY: yaml
yaml:
	cargo run --bin piccoloyaml $(filter-out $@,$(MAKECMDGOALS))