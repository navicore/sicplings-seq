# Hint: Sum of Squares

You have two values on the stack: `( x y )`

You need to square each and add them.

Think about the order of operations:
1. Square y (it's on top)
2. Swap to get x on top
3. Square x
4. Add the two squares

```seq
: sum-of-squares ( Int Int -- Int )
    square swap square i.+
;
```

In Seq, composition is just juxtaposition. Reading left to right:
- `square` squares y
- `swap` brings x to top
- `square` squares x
- `i.+` adds the two squares
