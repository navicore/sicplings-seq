# Hint: Greatest Common Divisor

Euclid's algorithm is elegant and the stack manipulation is simpler than factorial/fibonacci iterative.

## The algorithm

```
GCD(a, b) = a           if b = 0
GCD(a, b) = GCD(b, a%b) otherwise
```

## Structure

```seq
: gcd ( Int Int -- Int )
    # Stack: ( a b )
    dup 0 i.= if
        drop    # b is 0, return a
    else
        # recurse with ( b a%b )
    then
;
```

## The key: `tuck`

We need to transform `( a b )` into `( b a%b )` for the recursive call.

`tuck` copies the top under the second: `( a b -- b a b )`

Then `i.%` computes remainder: `( b a b -- b a%b )`

```seq
tuck i.%    # ( a b ) → ( b a b ) → ( b a%b )
gcd
```

## Full solution

```seq
: gcd ( Int Int -- Int )
    dup 0 i.= if
        drop
    else
        tuck i.%
        gcd
    then
;
```

## Why this is iterative

Even though it looks recursive, this generates an iterative process. Each call doesn't need to remember anything - it just passes new values. Seq (like Scheme) can optimize this as tail recursion.

## Trace: GCD(48, 18)

```
( 48 18 ) → tuck i.% → ( 18 12 )
( 18 12 ) → tuck i.% → ( 12 6 )
( 12 6 )  → tuck i.% → ( 6 0 )
( 6 0 )   → b=0, drop → ( 6 )
```

Answer: 6
