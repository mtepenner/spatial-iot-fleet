.PHONY: install validate server web

install:
	cd web-client && npm install

validate:
	cd server && cargo check
	cd web-client && npm install
	cd web-client && npm run build

server:
	cd server && cargo run

web:
	cd web-client && npm run dev
