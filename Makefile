# Note that this file doesn't recompile if the žlang's compiler code
# changes, only if the debug files do.
DEBUG_SOURCE_DIR=z/debug
ŽFILE=$(DEBUG_SOURCE_DIR)/testing.ž

LINKER=ld
BUILD_DIR=build

BIN=$(BUILD_DIR)/main
ASM_SOURCE=$(BUILD_DIR)/out.asm
OBJECT=$(BUILD_DIR)/out.o

all: build

run: build
	./$(BIN)

build: $(BIN)

$(BIN): $(OBJECT)
	$(LINKER) -o $@ $^

$(OBJECT): $(ŽFILE)
	cargo build
	./target/debug/z --asm $(ASM_SOURCE) -o $@ $^

clean:
	rm $(ASM_SOURCE) $(OBJECT) $(BIN)