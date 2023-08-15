# Note that this file doesn't recompile if the žlang's compiler code
# changes, only if the debug files do.
DEBUG_SOURCE_DIR=z/debug
ŽFILE=$(DEBUG_SOURCE_DIR)/testing.ž

LINKER=ld
ASSEMBLER=nasm
ASSEMBLER_FLAGS=-felf64 -g

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

$(OBJECT): $(ASM_SOURCE)
	$(ASSEMBLER) $(ASSEMBLER_FLAGS) -o $@ $^

$(ASM_SOURCE): $(ŽFILE)
	cargo run -p z -- $^
	# TODO: remove when compiler is fixed
	mv out.asm $(BUILD_DIR)

clean:
	rm $(ASM_SOURCE) $(OBJECT) $(BIN)