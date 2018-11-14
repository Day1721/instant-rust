all: compiler

compiler:
	cargo build
	cp src/scripts/insc_llvm.sh insc_llvm
	cp src/scripts/insc_jvm.sh insc_jvm
