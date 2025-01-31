# Segment Tree with Lazy Propagation to python

## Abstract

This is a Rust module that implements a segment tree for any type of operations with lazy propagation. The module aims to provide a fast implementation of the structure for general use in Python.

>[!NOTE]
This is amateur work, and the module is not optimized for performance. The module is a proof of concept and should not be used in production.

 


## Requirements
- maturin >= 1.8.1
- cargo >= 1.84.0
- python >= 3.10.14

## Installation

```bash
$ python -m venv .env
$ source .env/bin/activate
$ python -m pip install maturin
$ maturin develop
$ python python_impl/main.py

```


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
