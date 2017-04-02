# CollatzConjecture

## Information

Usage: `collatzconjecture METHOD NUMBER [OPTION]`

### Performance

Note that these are [unoptimized + debuginfo] timings, for now.

#### Synchronous (synchronous branch):

        time cargo run iterate 100000 // 10.63-11.37s

#### 1:1 concurrent (concurrency branch):

        time cargo run iterate 100000 // 15.55-17.01s

Too much overhead when using 1:1.

#### M:N concurrent (concurrency-rayon branch) with linear collection:

        time cargo run iterate 100000 // 13.10-13.69s

Less overhead, still bad. Need to use data structure which allows for 
concurrent collection, HashSet does not.

## Background

Conjecture: Every number chosen will converge to 1

Algorithm:

        if number is even
            divide by two
        else
            multiply by three and add one

## License
MIT
