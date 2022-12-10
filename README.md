# Little Man Computer Emulator

An LMC emulator which can be programmed with a basic assembly-like language, coded in rust.

## Sasm

| Instruction | Code | Function |
| --- | --- | --- |
| `ADD` | 1*xx* | Add the value in the given address (*xx*) to the value in the accumulator |
| `SUB` | 2*xx* | Subtract the value at the given address (*xx*) from the value in the accumulator |
| `STA` | 3*xx* | Store the contents in the accumulator at the given address (*xx*) |
| `LDA` | 5*xx* | Load the value at the given address (*xx*) to the accumulator |
| `BRA` | 6*xx* | Set the program counter to the given address (*xx*), thereby jumping to that instruction |
| `BRZ` | 7*xx* | If the accumualtor contains the value `0`, set the program counter to the given address (*xx*) |
| `BRP` | 8*xx* | If the accumualtor contains a positive value, set the program counter to the given address (*xx*) |
| `INP` | 901 | Get an input value from the user and store it in the accumulator |
| `OUT` | 902 | Print out the value currently stored in the accumulator |

Comments begin after a `;` and are ignored.

Labels are defined with `,` and can be referenced with `'`

They are a WIP, and cannot be used with instructions like `DAT`, but its a start.

Example use:

A simple program that counts down from a given number by using a given step value

(Obviosuly this isnt practical because entering, ex. 5 & 2 would lead to an infinite (?) loop, but its a demonstration its fine)

```sasm
INP
STA 99 ; get count
INP
STA 98 ; get dec

LDA 99 ,loop ; Load count and set the loop label
SUB 98       ; Subtract step
OUT
BRZ 'end ; If result == 0, jump to end label
STA 99   ; Update count
BRA 'loop ; Jump to loop label

HLT ,end ; Declare end label
```

A simple program to add 2 numbers:

```sasm
INP
STA 99 ; Store input in reg. 99 (lhs)
INP
STA 98 ; Store input in reg. 98 (rhs)

LDA 99 ; Load reg. 99 (rhs)
ADD 98 ; Add reg. 98 (lhs) to accumulator (rhs)
OUT ; Output accumulator (result)

HLT ; Exit
```

## Todo

- Data instruction
- Better error handling (parsing & lexing)

## License

For what It's worth, this is committed to the public domain with the Unlicense.
