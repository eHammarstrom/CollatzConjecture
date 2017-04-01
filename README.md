# CollatzConjecture

## Information

Usage: `collatzconjecture METHOD NUMBER [OPTION]`

### Performance

Note that these are [unoptimized + debuginfo] timings, for now.

Non-concurrent:
`time cargo run iterate 100000` 10.63-11.37s

1:1 concurrent:
`time cargo run iterate 100000` 15.55-17.01s

Too much overhead when using 1:1.

## Background

Conjecture: Every number chosen will converge to 1

Algorithm:

        if number is even
            divide by two
        else
            multiply by three and add one

## License
MIT
