
.PHONY: all
all: ed25519-donna bench

bench:
	cargo bench --features="bench" | tee benchmark-results

ed25519-donna:
	cd ed25519-donna; \
		gcc ed25519.c -fstack-protector-strong -fPIE -D_FORTIFY_SOURCE=2 -Wformat -Wformat-security -Werror=format-security -m64 -O3 -c -DED25519_SSE2 -o ed25519donna

test-ed25519-donna:
	cd ed25519-donna; \
		gcc ed25519.c test.c -fstack-protector-strong -fPIE -D_FORTIFY_SOURCE=2 -Wformat -Wformat-security -Werror=format-security -m64 -O3 -c -DED25519_SSE2 -DED25519_TEST -o test_ed25519_donna; \
		./test_ed25519_donna
	cd ed25519-donna; \
		gcc ed25519.c test-internals.c -fstack-protector-strong -fPIE -D_FORTIFY_SOURCE=2 -Wformat -Wformat-security -Werror=format-security -m64 -O3 -c -DED25519_SSE2 -DED25519_TEST -o test_ed25519_donna_internals; \
		./test_ed25519_donna_internals

clean:
	-rm benchmark-results
	-cd ed25519-donna; rm *.o ed25519_donna test_ed25519_donna test_ed25519_internals
