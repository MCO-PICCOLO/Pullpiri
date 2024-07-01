# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

.PHONY: default build
build:
	cargo build --manifest-path=src/Cargo.toml

.PHONY: release
release:
	cargo build --manifest-path=src/Cargo.toml --release

.PHONY: tool
tool:
	cargo build --manifest-path=src/tools/Cargo.toml

.PHONY: clean
clean:
	cargo clean --manifest-path=src/Cargo.toml
	cargo clean --manifest-path=src/tools/Cargo.toml

# Section for podman-kube workload - START
.PHONY: image
image:
	podman build -t piccolo:1.0 -f containers/Dockerfile .
	podman build -t piccolo-gateway:1.0 -f containers/Dockerfile-gateway .

.PHONY: install
install:
	-mkdir /root/piccolo_yaml
	-mkdir /etc/containers/systemd/piccolo/
	-mkdir /etc/containers/systemd/piccolo/example
	-cp -r ./piccolo.ini /etc/containers/systemd/piccolo/
	-cp -r ./containers/piccolo.* /etc/containers/systemd/piccolo/
	-cp -r ./etcd-data /etc/containers/systemd/piccolo/etcd-data/
	systemctl daemon-reload
	systemctl start piccolo

.PHONY: uninstall
uninstall:
	-systemctl stop piccolo
	-rm -rf /etc/containers/systemd/piccolo/
	systemctl daemon-reload

.PHONY: tinstall
tinstall:
	-mkdir /etc/containers/systemd/piccolo-test/
	-cp -r ./examples/version-display/scenario/* /etc/containers/systemd/piccolo/example/
	-cp -r ./examples/version-display/qt-msg-sender/qt-sender.* /etc/containers/systemd/piccolo-test/
	systemctl daemon-reload
	systemctl start qt-sender

.PHONY: tuninstall
tuninstall:
	-systemctl stop qt-sender
	-rm -rf /etc/containers/systemd/piccolo-test/
	-systemctl stop version-display.service
	-rm -rf /etc/containers/systemd/version-display.kube
	systemctl daemon-reload
# Section for podman-kube workload - END
