.PHONY: watch-tests

watch-tests:
	cargo watch -x "test -- --nocapture"