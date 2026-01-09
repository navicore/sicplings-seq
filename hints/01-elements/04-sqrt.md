# Hint: Square Root by Newton's Method

This is the most complex exercise so far. Break it into pieces!

## improve

```seq
: improve ( guess x -- improved-guess )
    # new = (guess + x/guess) / 2
    over f./       # x/guess
    swap f.+       # guess + x/guess
    2.0 f./        # (guess + x/guess) / 2
;
```

## good-enough?

```seq
: good-enough? ( guess x -- Bool )
    over dup f.*   # guess^2
    swap f.-       # guess^2 - x
    f-abs          # |guess^2 - x|
    0.001 f.<      # < 0.001?
    nip            # clean up stack
;
```

You'll need a helper for floating-point absolute value:
```seq
: f-abs ( Float -- Float )
    dup 0.0 f.< if 0.0 swap f.- then
;
```

## The main loop

Use a while loop to keep improving until good enough:
```seq
: sqrt ( Float -- Float )
    1.0 swap    # ( guess x )
    [ 2dup good-enough? not ]    # condition
    [ 2dup improve rot drop swap ]  # body: improve and rearrange
    while
    drop    # return guess
;
```

The stack juggling in the while body is tricky. Take it step by step!
