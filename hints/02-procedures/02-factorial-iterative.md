# Hint: Factorial (Iterative)

This exercise introduces the challenge of threading multiple pieces of state through recursion in a stack-based language.

## The challenge

We have three values `( product counter max-count )` and need to transform them to `( product*counter counter+1 max-count )` for the recursive call.

## Termination check

Use `2dup i.>` to check if counter > max-count:
```seq
2dup i.> if
    # counter > max-count: return product
    drop drop    # discard counter and max-count
else
    # continue iterating
then
```

## The stack shuffle

This is the tricky part. Starting with `( product counter max-count )`:

```seq
rot rot          # ( max-count product counter )
tuck i.*         # ( max-count counter product*counter )
swap             # ( max-count product*counter counter )
1 i.+            # ( max-count product*counter counter+1 )
rot              # ( product*counter counter+1 max-count )
```

### Why `rot rot`?

`rot` rotates the third element to the top: `( a b c -- b c a )`

`rot rot` effectively does the reverse: `( a b c -- c a b )`

This puts max-count at the bottom where we want it while we work with product and counter.

### Why `tuck`?

`tuck` copies the top under the second: `( a b -- b a b )`

With `( product counter )`, `tuck` gives `( counter product counter )`.
Then `i.*` multiplies: `( counter product*counter )`.

## Full solution

```seq
: fact-iter ( Int Int Int -- Int )
    # Stack: ( product counter max-count )
    2dup i.> if
        drop drop
    else
        rot rot          # ( max-count product counter )
        tuck i.*         # ( max-count counter product*counter )
        swap             # ( max-count product*counter counter )
        1 i.+            # ( max-count product*counter counter+1 )
        rot              # ( product*counter counter+1 max-count )
        fact-iter
    then
;
```
