# bf
Dirt simple BrainFuck interpreter

Features:  
- jump table for handling loops.  
- cells are wrapped on over/under flow  
- input handled with enter but may require a ctrl+d to end input.  
- 30000 cell default memory, data pointer starts at 15000.

I have tested this with dozens of BrainFuck scripts and 99.99% of them run perfectly.  
This is an unoptimized implementation and does not check for mismatched brackets.

Run with:
```
cargo build
./bf examples/mandelbrot.b
```

