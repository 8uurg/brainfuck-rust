# Brainfuck Interpreter in Rust
In order to try out a new language, why not write an interpreter?

Turns out it is a pretty fun way to learn parts of a programming language.
The current code uses `match` statements, `enums`, and shows off `iterators`.

A few optimizations can still be performed, and will probably be done to play around with it:

[ ] Group instructions together <br/>
[ ] Precompute pointers for loops, rather than doing this on the fly. <br/>
[ ] In some special cases of loops

## Measurements
`measure-command { cargo run .\examples\mandelbrot.bf | Out-Default }`
```
Days              : 0
Hours             : 0
Minutes           : 15
Seconds           : 12
Milliseconds      : 514
Ticks             : 9125143247
TotalDays         : 0.0105615083877315
TotalHours        : 0.253476201305556
TotalMinutes      : 15.2085720783333
TotalSeconds      : 912.5143247
TotalMilliseconds : 912514.3247
```

`measure-command { cargo run --release .\examples\mandelbrot.bf | Out-Default }`
```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 29
Milliseconds      : 658
Ticks             : 296585256
TotalDays         : 0.000343269972222222
TotalHours        : 0.00823847933333333
TotalMinutes      : 0.49430876
TotalSeconds      : 29.6585256
TotalMilliseconds : 29658.5256
```

