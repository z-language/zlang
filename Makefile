# Note that this file doesn't recompile if the žlang's compiler code
# changes, only if the debug files do.
DEBUG_SOURCE_DIR=z/debug
ŽFILE=$(DEBUG_SOURCE_DIR)/testing.ž

LINKER=ld
ASSEMBLER=nasm
ASSEMBLER_FLAGS=-felf64 -g

BIN=main
ASM_SOURCE=out.asm
OBJECT=out.o

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

clean:
	rm $(ASM_SOURCE) $(OBJECT) $(BIN)