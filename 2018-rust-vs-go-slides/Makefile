
build:
	@rm -rf docs ; \
	cargo web build --release --target=wasm32-unknown-unknown ; \
	mkdir docs ; \
	cp target/wasm32-unknown-unknown/release/rust-vs-go-slides.js docs/ ; \
	cp target/wasm32-unknown-unknown/release/rust-vs-go-slides.wasm docs/ ; \
	cp static/* docs/ ; \
	sed -i  "s/js\/app.js/rust-vs-go-slides.js/g" docs/index.html
