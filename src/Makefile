CFLAGS = -fpic
DEBUG_DIR = ../target/debug/deps
RELEASE_DIR = ../target/release/deps
MKDIR_P = mkdir -p

all:
	${MKDIR_P} ${DEBUG_DIR}
	${CC} ${CFLAGS} -Wall probe.c -c -o probe.o
	${AR} rcs ${DEBUG_DIR}/libprobe.a probe.o 
	rm probe.o
	cargo +nightly build

release: dirs
	${MKDIR_P} ${RELEASE_DIR}
	${CC} ${CFLAGS} -w probe.c -c -o probe.o
	${AR} rcs ${RELEASE_DIR}/libprobe.a probe.o
	rm probe.o
	cargo +nightly build --release

clean:
	rm -rf ../target
	rm *.run *.a *.o ${DEBUG_DIR}/*.a ${RELEASE_DIR}/*.a 