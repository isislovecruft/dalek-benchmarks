
.PHONY: all
all: fetch-submodules bench-all

fetch-submodules:
	git submodule update --init

bench-all: bench-ed25519-dalek bench-ed25519-donna

bench-ed25519-dalek:
	$(MAKE) -C ed25519-dalek bench

clean-ed25519-dalek:
	$(MAKE) -C ed25519-dalek clean

bench-ed25519-donna:
	$(MAKE) -C ed25519-donna bench

clean-ed25519-donna:
	$(MAKE) -C ed25519-donna clean

clean: clean-ed25519-dalek clean-ed25519-donna
	-rm benchmark-results
