# Hint: Factorial (Recursive)

The pattern is straightforward - this is the simplest recursive exercise.

## Structure

```seq
: factorial ( Int -- Int )
    dup 1 i.<= if
        # base case: return 1
    else
        # recursive case: n * factorial(n-1)
    then
;
```

## Key insight

In Scheme, `(* n (factorial (- n 1)))` computes `(n-1)!` first, then multiplies.

In Seq, we need to:
1. Keep n on the stack
2. Compute factorial(n-1)
3. Multiply

```seq
dup 1 i.- factorial i.*
```

Trace through:
- `dup` → ( n n )
- `1 i.-` → ( n n-1 )
- `factorial` → ( n (n-1)! )
- `i.*` → ( n*(n-1)! )

## Full solution

```seq
: factorial ( Int -- Int )
    dup 1 i.<= if
        drop 1
    else
        dup 1 i.- factorial i.*
    then
;
```
