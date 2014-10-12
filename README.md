
StackVec
========

A wrapper around the Rust fixed-size array for `Vec` like behaviour on the stack.

```Rust

// Create an empty StackVec with a max size of 32 elems.
// Note: you can use any power of 2 size up to 1024.
let mut vec: StackVec<N32<uint>> = StackVec::new();

// Push onto the end like a normal Vec.
for i in range(0u, 32) {
    vec.push(i)
}

// Pushing past the max will cause failure.
vec.push(0u); // error...

```
