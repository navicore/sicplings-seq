# Chapter 1.1: The Elements of Programming

> "The acts of the mind, wherein it exerts its power over simple ideas, are chiefly these three:
> 1. Combining several simple ideas into one compound one...
> 2. Bringing two ideas together... to get a view of any relation...
> 3. Separating them from all other ideas that accompany them..."
> — John Locke, *An Essay Concerning Human Understanding*

SICP begins with three mechanisms for combining simple ideas:

1. **Primitive expressions** - the simplest entities (numbers, built-in operations)
2. **Means of combination** - building compound elements from simpler ones
3. **Means of abstraction** - naming compound elements as units

In Scheme, these are numbers, procedure application, and `define`.

In Seq:
- **Primitives**: numbers (`42`), built-in words (`i.+`, `i.*`)
- **Combination**: function composition (words composed by juxtaposition)
- **Abstraction**: word definitions (`: name ... ;`)

## The Key Insight

In Scheme: `(+ 1 2)` - operator first, then operands
In Seq: `1 2 i.+` - operands first, then operator

This isn't just syntax - it reflects a different model of computation. In Seq, data flows through a pipeline of transformations. Each word takes from the stack and leaves results for the next.

## Exercises

These exercises establish the fundamentals before we tackle Newton's method.
