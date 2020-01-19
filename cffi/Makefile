TARGET:=../target/debug

.PHONY: build_c
build_c:
	gcc -c -o ${TARGET}/native/fib.o c_src/fib.c
	ar r ${TARGET}/deps/libfib.a ${TARGET}/native/fib.o

run_with_option:
	RUSTFLAGS="-L native=$(PWD)/../target/debug/deps" cargo run