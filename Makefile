.PHONY: terrctl.install
terrctl.install:
	go get -v github.com/fastly/terrctl/terrctl

.PHONY: deploy
deploy: terrctl.install
	terrctl -loglevel 0 {src,assets}/**
