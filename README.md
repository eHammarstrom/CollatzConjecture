# CollatzConjecture

## Information

Usage: `collatzconjecture METHOD NUMBER [OPTION]`

### Performance

Compile rust [optimized] + these flags: `RUSTFLAGS="-C target-cpu=native -C target-feature=+avx,+mmx,+popcnt,+sse,+sse2,+sse3,+ssse3"`

Synchronous solution: `[0.12s .. 0.16s]`

## Background

Conjecture: Every number chosen will converge to 1

Algorithm:

        if number is even
            divide by two
        else
            multiply by three and add one

## License

MIT
