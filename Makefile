all: deploy

.PHONY: deploy
deploy: terrctl.install
	terrctl -loglevel 0 {src,assets}/**

.PHONY: terrctl.install
terrctl.install:
	go get -v github.com/fastly/terrctl/terrctl
