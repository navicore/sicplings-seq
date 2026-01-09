# Hint: Square

In Scheme: `(* x x)` - multiply x by itself.

In Seq, you need to:
1. Duplicate the value (so you have two copies)
2. Multiply them

```seq
: square ( Int -- Int )
    dup i.*
;
```

`dup` copies the top of stack, `i.*` multiplies the top two integers.
