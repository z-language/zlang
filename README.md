# ŽLANG COMPILER

This is a parser and a compiler for the Ž programming language.
It's still in its early stages.

Features:
- [x] Tokenizer
- [x] Parser
- [x] Compiler (WIP)

TODO:
- [ ] Implement all operators
- [ ] Compiler optimizations
- [ ] CODE CLEANUP!
- [ ] Floating point numbers
- [ ] Type conversions
- [ ] Negative numbers

## A QUICK INTRODUCTION

### Defining functions
You define a function with the "fun" keyword. Note that every program needs a main function.
```kotlin
fun foo(x: int, y: float) -> int {}
//  ^^^ This function takes in two
//      argumments and returns an int.

fun foo2() {}
//  ^^^^ By default functions return "none".
```
## Defining variables
You define a variable with the "var" keyword. By default, all variables are immutable and to make a variable mutable add the "mut" keyword after the var keyword. Immutable variables have to be assigned at declaration, while mutable variables will, by default, be set to "none".
```kotlin
var i = 0
var mut i = 3.5
var i = 5_000_000
//      ^^^^^^^^^
// you can also format numbers to your liking

var i
//  ^ this returns an error
var mut i
//  ^^^ here, i is "none"
```

## Scope
A scope is just a block of code contained in it's own scope. To define it, write your scoped code inside a pair of curly brackets.
```kotlin
{
    var i = 5
    // you can use i here

    {} // a scope in a scope
}
// but not here
```

## Loops
The most basic form of looping is an infinite loop. To define a loop use the "loop" keyword, followed by a scope (look at the scope chapter).
Loops can be exited with the "break" keyword.
```rust
loop {}

loop {
    if something {
        break
    }
}
```

## If statements
```kotlin
if cond {}
else {}

if cond {}
else if cond {}
else {}

if cond {
    // ...
}
```

## Strings
Strings are not fully implemented yet, but you can already use them to some extend.
```kotlin
var name = "mark"
var greeting = "hello \"mark\""
//                    ^ you can use \ to escape
var path = "C:\\Drive\\something"
//            ^ you can also escape an escape
```
### Function calls
Function calls work as you might expect.
```rust
foo(5, 6)
test()
nice(5 + 9.2)
```

### Inline Assembly
To write inline assembly, use the \_\_asm__ keyword. Call it like a function and pass strings as it's arguments. To reference a variable from your code, use the dollar ($) symbol. This is a hello world example:
```kotlin
var message = "Hello, World\n"
var message_len = 13
__asm__(
    "mov rax, 1",
    "mov rdi, 1",
    "mov rsi, $message",
    "mov rdx, $message_len",
    "syscall",
)
```
Output:
```
Hello, World
```

## Planned feautres

### Structs
```go
struct Person {
    name: string
    age: int
    married: bool
}
```

### For loops

### Modules
```kotlin
import foo

fun main() {
    foo.test()
}
```

### Deref
```kotlin
var name = "Mark"
assert_eq(*(name+2), "r")
```

### Errors

### Arrays
```kotlin
var nums = [2, 4, 6]
assert_eq(nums[1], 4)
```
