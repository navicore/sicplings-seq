# Hint: Fibonacci (Tree Recursion)

This is similar to factorial but with TWO recursive calls - creating tree recursion.

## Structure

You need nested conditionals for the two base cases:

```seq
: fib ( Int -- Int )
    dup 0 i.= if
        # fib(0) = 0
    else
        dup 1 i.= if
            # fib(1) = 1
        else
            # fib(n) = fib(n-1) + fib(n-2)
        then
    then
;
```

## The recursive case

The challenge: compute both `fib(n-1)` and `fib(n-2)`, then add them.

```seq
dup 1 i.- fib    # ( n fib(n-1) )
swap 2 i.- fib   # ( fib(n-1) fib(n-2) )
i.+              # ( fib(n-1)+fib(n-2) )
```

Trace through with n=5:
- Start: ( 5 )
- `dup 1 i.-` → ( 5 4 )
- `fib` → ( 5 fib(4) )
- `swap` → ( fib(4) 5 )
- `2 i.-` → ( fib(4) 3 )
- `fib` → ( fib(4) fib(3) )
- `i.+` → ( fib(4)+fib(3) )

## Full solution

```seq
: fib ( Int -- Int )
    dup 0 i.= if
        drop 0
    else
        dup 1 i.= if
            drop 1
        else
            dup 1 i.- fib
            swap 2 i.- fib
            i.+
        then
    then
;
```

## Note on efficiency

This is exponentially slow! Try computing `30 fib` - it will take a very long time. The iterative version in the next exercise solves this.
