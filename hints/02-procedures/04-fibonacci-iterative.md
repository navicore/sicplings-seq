# Hint: Fibonacci (Iterative)

Like factorial-iterative, this threads state through recursion. The transformation is: `( a b count ) → ( a+b a count-1 )`

## Termination

When count reaches 0, return b:
```seq
dup 0 i.= if
    drop nip    # discard count and a, keep b
else
    # iterate
then
```

`nip` removes the second element: `( a b -- b )`

## The stack transformation

Starting with `( a b count )`, we need `( a+b a count-1 )`:

```seq
1 i.-            # ( a b count-1 )
rot rot          # ( count-1 a b )
over             # ( count-1 a b a )
i.+              # ( count-1 a a+b )
swap             # ( count-1 a+b a )
rot              # ( a+b a count-1 )
```

### Trace with ( 1 0 3 )

- Start: ( 1 0 3 ) where a=1, b=0, count=3
- `1 i.-` → ( 1 0 2 )
- `rot rot` → ( 2 1 0 )
- `over` → ( 2 1 0 1 )
- `i.+` → ( 2 1 1 )  ← 0+1=1
- `swap` → ( 2 1 1 )
- `rot` → ( 1 1 2 )

Result: new a=1, new b=1, count=2. Correct!

## Full solution

```seq
: fib-iter ( Int Int Int -- Int )
    dup 0 i.= if
        drop nip
    else
        1 i.-            # ( a b count-1 )
        rot rot          # ( count-1 a b )
        over             # ( count-1 a b a )
        i.+              # ( count-1 a a+b )
        swap             # ( count-1 a+b a )
        rot              # ( a+b a count-1 )
        fib-iter
    then
;
```

Now `30 fib` computes instantly instead of taking forever!
