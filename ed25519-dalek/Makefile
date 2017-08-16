
.PHONY: all
all: bench

bench:
	cd ed25519-dalek; \
		cargo bench --features="bench nightly" | tee ../benchmark-results

clean:
	-rm benchmark-results
