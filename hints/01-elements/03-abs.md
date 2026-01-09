# Hint: Absolute Value

Check if the number is negative, and if so, negate it.

```seq
: abs ( Int -- Int )
    dup 0 i.< if
        0 swap i.-    # 0 - n = -n
    then
;
```

Key insights:
- `dup 0 i.<` checks if the value is less than 0, leaving the original value below
- `if ... then` executes the branch only if true
- `0 swap i.-` computes 0 - n, which negates the number
