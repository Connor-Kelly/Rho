

declare i32 @printf(i8* noundef, ...)
; Function Attrs: noinline nounwind optnone uwtable
define i32 @main() #0 {
    ; allocates a string
    %strArr = alloca [4 x i8], align 1
    ; stores a value in that string
    store [4 x i8] c"%d\0A\00", [4 x i8]* %strArr
    ; gets a pointer to that string
    %str = getelementptr [4 x i8], [4 x i8]* %strArr, i32 0, i32 0

    ; gets an int
    %myInt = add i32 69, 0

    call i32 (i8*, ...) @printf(i8* %str, i32 %myInt)
    ret i32 0
}
