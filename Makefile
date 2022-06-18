.PHYNO: build
build:
	cargo build

TOKEN = 
.PHYNO: test
test:
ifeq ($(TOKEN),)
	@echo "Usage: make test TOKEN=<token>"
else
	YUQUE_TOKEN=$(TOKEN) cargo test \
		--package yuque \
		--lib -- --exact --nocapture
endif
