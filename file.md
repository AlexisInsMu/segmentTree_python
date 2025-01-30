# Segment Tree with Lazy Propagation to python

## Abstract

This is a Rust module that implements a segment tree for any type of operations with lazy propagation. The module aims to provide a fast implementation of the structure for general use in Python.

## Usage

```python
from segment_tree import SegmentTree

# Create a segment tree with 10 elements
st = SegmentTree(10)

# Update the range [0, 5) with 1
st.update(0, 5, 1)
   
# Query the range [0, 5)
print(st.query(0, 5)) # 5
```
