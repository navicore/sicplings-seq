# Hint: Fast Exponentiation

This combines conditionals with recursion. The helper words `even?` and `square` are provided - you just need to implement `fast-expt`.

## Three cases

```seq
: fast-expt ( Int Int -- Int )
    # Stack: ( b n )
    dup 0 i.= if
        # n=0: return 1
    else
        dup even? if
            # n even: square(fast-expt(b, n/2))
        else
            # n odd: b * fast-expt(b, n-1)
        then
    then
;
```

## Case 1: n = 0

```seq
drop drop 1    # discard b and n, return 1
```

## Case 2: n is even

Compute `fast-expt(b, n/2)` then square it:

```seq
2 i./           # ( b n/2 )
fast-expt       # ( result )
square          # ( result^2 )
```

Note: We don't need b after the recursive call because fast-expt consumes it.

## Case 3: n is odd

Compute `b * fast-expt(b, n-1)`:

```seq
1 i.-           # ( b n-1 )
over swap       # ( b b n-1 )
fast-expt       # ( b result )
i.*             # ( b*result )
```

The `over swap` pattern duplicates b and puts it under for later multiplication.

## Full solution

```seq
: fast-expt ( Int Int -- Int )
    dup 0 i.= if
        drop drop 1
    else
        dup even? if
            2 i./
            fast-expt
            square
        else
            1 i.-
            over swap
            fast-expt
            i.*
        then
    then
;
```

## Trace: 2^10

```
fast-expt(2, 10)  → even, fast-expt(2, 5)
fast-expt(2, 5)   → odd, 2 * fast-expt(2, 4)
fast-expt(2, 4)   → even, fast-expt(2, 2)
fast-expt(2, 2)   → even, fast-expt(2, 1)
fast-expt(2, 1)   → odd, 2 * fast-expt(2, 0)
fast-expt(2, 0)   → 1

Working back up:
2 * 1 = 2
square(2) = 4
square(4) = 16
2 * 16 = 32
square(32) = 1024
```

Only 5 operations instead of 10!
