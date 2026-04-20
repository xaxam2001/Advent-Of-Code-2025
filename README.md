# Advent-Of-Code-2025
Rust implementation of the advent of code 2025

# Notes
## d1p1

### v2
used Vec<u8> instead of &str to iterate over lines.

### v3
removed Vec<u8> to avoid allocating on the heap.
(difference noticeable when loading heavy input)

### v4
getting rid of rem_euclid to use the standard % instead.

### v5
using `as usize` instead of `usize::from` (supposed to make the compiler dropping extra sign checks so usually less safe) 

### v6
no significantly faster to use `as usize` so back to previous cast
looks a bit like dark magic but should make the compiler do less bounds check when calling line[1..] and line [0]
It didn't work and it seems that it's because the implicit compiler bound check is a cold path (should almost never happen)
and the `continue` is a hot path means that it's actively executed each loop

### v7
removing the `panic!` macro to see if it makes it faster
not sure if it really made slower than the v4 (bc it's the same as v4 except the panic and it quite slower or is it the compute I'm using?)

## d1p2

### v2
used Vec<u8> instead of &str to iterate over lines. like in d1p1_v3

### v3
splitting the computation between L and R bc the R case is simpler and less costly

### v4
optimizing the subtracting by getting rid of `rem_euclid`

## d2p1

### v2
used u8 instead of str for parsing

### v3
use maths to detect if both part are equal instead of reconverting to string

## d2p2

### v2
used u8 and used math to see if part were equal

### v3
used ten power lookup table rather than computing with pow

### v4
used only prime dividers because if the id isn't composed of similar blocks when divided into 2 blocks then it wont be when divided into 4, 8 or 10 blocks so we can skip

### v5
removed the for parts loop, now we remove the first block and the last block and compare them, if there are the same then the blocks are similar


## d3p1

### v2
using iterators instead of nth

### v3
using u8 list instead of str

### v4
getting rid of the double loop, refined logic

### v5
try to work with u8 (slower bc conversion to usize mandatory and done twice in the loop instead of once)

### v6 (slower)
tried to get rid of the split method, got things worth because split is highly optimized and bases on memchr which uses
SMID (Single Instruction, Multiple Data)


## d8p1

### v2
compute the points coordinates value ahead one time for all

### v3
used squared distance to get rid of the computation of a square root