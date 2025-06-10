CARGO="cargo"

build:
	@$(CARGO) build --release --target x86_64-unknown-linux-musl

test:
	@$(CARGO) test

deb: build
	@$(CARGO) deb --target x86_64-unknown-linux-musl

rpm: build
	@$(CARGO) generate-rpm --target x86_64-unknown-linux-musl

clean:
	@$(CARGO) clean

.PHONY: build
