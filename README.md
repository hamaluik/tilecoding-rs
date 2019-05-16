# Tilecoding (Rust Implementation)

[![Crates.io](https://img.shields.io/crates/v/tilecoding.svg)](https://crates.io/crates/tilecoding)
[![Docs](https://docs.rs/tilecoding/badge.svg)](https://docs.rs/tilecoding)
[![Build Status](https://travis-ci.org/hamaluik/tilecoding-rs.svg?branch=master)](https://travis-ci.org/hamaluik/tilecoding-rs)

This is a Rust version of the Tile Coding Software developed by [Dr. Richard S. Sutton](http://richsutton.com/index.html) available on his website at: http://incompleteideas.net/tiles/tiles3.html. It is strongly suggested to read that page in full as it describes the reasoning behind this software, how to use it, and gives examples.

To quote the introduction there:

> Here we describe software implementing the core part of the idea of tile coding as described in [Section 9.5.4](http://www.incompleteideas.net/book/RLbook2018.pdf#page=239) of the reinforcement-learning textbook by Sutton and Barto. That section should be read and fully understood before reading this manual or using this software. The software is currently available in Python and Common Lisp. An implementation in C or C++ would be a welcome contribution.
>
> Tile coding is a way of representing the values of a vector of continuous variables as a large binary vector with few 1s and many 0s. The binary vector is not represented explicitly, but as a list of the components that are 1s. The main step is to partition, or tile, the continuous space multiple times and select one tile from each tiling, that corresponding the the vector's value. Each tile is converted to an element in the big binary vector, and the list of the tile (element) numbers is returned as the representation of the vector's value. Tile coding is recommended as a way of applying online learning methods to domains with continuous state or action variables.
>
> The tile-coding software evolved from software developed at the University of New Hampshire for a related but more specialized use called "[CMAC](http://en.wikipedia.org/wiki/Cerebellar_Model_Articulation_Controller)" (see the [external documentation by Miller and Glanz](http://incompleteideas.net/tiles/tilesUNHdoc.pdf)). Here we separate tile coding from full CMACs, which allows it to be used more flexibly. The current software is also simplified and streamlined consistent with its use in reinforcement learning applications. For example, our code is only one page in length. 

This version provides two implementations:

1. Using an index-hash-table (IHT)
2. Without using an IHT (slightly faster, not as "safe" as using the IHT as a collision table)

## API

Whether using an IHT or not, the general format of the `tiles` function is the same:

```rust
fn IHT::tiles(&mut self, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<usize>;
fn tiles(size: usize, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<usize>;
```

> An IHT must be initialized with a `size`, which is the upper bounds of all returned indices. When not using an IHT, you must provide this parameter with each call. For extremely large problems, you may not be able to use an IHT, which is why the second form is provided.
> 
> The `num_tilings` argument should be a power of two greater or equal to four times the number of floats.
> 
> The `floats` argument is a list of floating-point numbers to be tiled.
> 
> The `ints` argument is optional, and when it is provided it is a list of integers that will also be tiled; all distinct integers will result in different tilings. In reinforcement learning, discrete actions are often provided here.

## Examples


### Simple

```rust
// initialize an index-hash-table with size `1024`
let mut iht = IHT::new(1024);

// find the indices of tiles for the point (x, y) = (3.6, 7.21) using 8 tilings:
let indices = iht.tiles(8, &[3.6, 7.21], &[]);

// this is the first time we've used the IHT, so we will get the starting tiles:
assert_eq!(indices, vec![0, 1, 2, 3, 4, 5, 6, 7]);

// a nearby point:
let indices = iht.tiles(8, &[3.7, 7.21], &[]);

// differs by one tile:
assert_eq!(indices, vec![0, 1, 2, 8, 4, 5, 6, 7]);

// and a point more than one away in any dim
let indices = iht.tiles(8, &[-37.2, 7.0], &[]);

// will have all different tiles
assert_eq!(indices, vec![9, 10, 11, 12, 13, 14, 15, 16]);
```
