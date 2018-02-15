# Slice Stripes

A fairly trivial crate with a pair of iterators for reading slices (and coerced Vecs) stripe-wise rather than chunk-wise.

Mostly upped as a note-to-self about using trait extensions on core types.

E.g.

```
use slice_stripes::Striped;

let x = [1,2,3,
         4,5,6,
         7,8,9];

println!("{:?}", x.stripes(3).next().unwrap().collect());
```
ought to output "[1,4,7]"
