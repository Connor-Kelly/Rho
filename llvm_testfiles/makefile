filename="printInt"
libc="/usr/lib/gcc/x86_64-linux-gnu/11/../../../x86_64-linux-gnu/"
# Final Linking (Optional):
# 	The clang or another linker is used to perform the final linking. 
# 	This step might involve linking with external libraries and 
# 	resolving symbols.
create-executable: compile-to-object 
	clang -v -o ${filename}   ${filename}.o -fPIE

# Object File Generation:
# 	The llc tool is used to generate object files (.o) from LLVM bitcode. 
# 	This step involves selecting a target architecture and emitting 
# 	native machine code.
compile-to-object: compile-to-bytecode
	llc -filetype=obj -o ${filename}.o ${filename}.bc

# Compilation to LLVM Bitcode:
# 	Each source file (.ll, .c, etc.) is compiled to LLVM bitcode (.bc) using 
# 	the llvm-as tool. This results in one bitcode file per source file.
compile-to-bytecode:
	llvm-as -o ${filename}.bc ${filename}.ll


dump: compile-to-object
	objdump -x ${filename}.o

# alternatively: 
# readelf -a your_file.o
# |
# nm -a your_file.o

run: create-executable
	./${filename}

clean: 
	rm ${filename} ${filename}.o ${filename}.bc

# all: create-executable

