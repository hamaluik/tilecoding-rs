// benchmarks only available on nightly for now
//#![feature(test)]

use std::collections::HashMap;

/// Convenience function for hashing a hashable object using the std hashmap's default hasher
pub fn base_hash<H>(obj: H) -> usize
where
    H: std::hash::Hash,
{
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as usize
}

/// An index-hash-table, or IHT. It will allow to collect tile indices up to a
/// certain size, after which collisions will start to occur. The underlying storage
/// is a HashMap
pub struct IHT {
    size: usize,
    overfull_count: usize,
    dictionary: HashMap<Vec<isize>, usize>,
}

impl IHT {
    /// Create a new IHT with the given size. The `tiles` function will never
    /// report an index >= this size.
    pub fn new(size: usize) -> IHT {
        IHT {
            size,
            overfull_count: 0,
            dictionary: HashMap::with_capacity(size),
        }
    }

    fn get_index(&mut self, obj: Vec<isize>) -> usize {
        // store the count for later use
        let count = self.dictionary.len();

        // use the entry api on hashmaps to improve performance
        use std::collections::hash_map::Entry;
        match self.dictionary.entry(obj) {
            // if the object already exists in the hashmap, return the index
            Entry::Occupied(o) => *o.get(),

            Entry::Vacant(v) => {
                // the object isn't already stored in the dictionary
                if count >= self.size {
                    // if we're full, allow collisions (keeping track of this fact)
                    self.overfull_count += 1;
                    base_hash(v.into_key()) % self.size
                } else {
                    // otherwise, just insert into the dictionary and return the result
                    *v.insert(count)
                }
            }
        }
    }

    fn get_index_read_only(&mut self, obj: Vec<isize>) -> Option<usize> {
        use std::collections::hash_map::Entry;
        match self.dictionary.entry(obj) {
            Entry::Occupied(o) => Some(*o.get()),
            Entry::Vacant(_) => None
        }
    }

    /// Convenience function to determine if the IHT is full. If it is, new tilings will result in collisions rather than new indices.
    pub fn full(&self) -> bool {
        self.dictionary.len() >= self.size
    }

    /// Convenience function to determine how full the IHT is. The maximum value will be the IHT size
    pub fn count(&self) -> usize {
        self.dictionary.len()
    }

    /// Convenience function get the size of the IHT, in case you forgot what it was
    pub fn size(&self) -> usize {
        self.size
    }

    /// This function takes a series of floating point and integer values, and encodes them as tile indices using the underlying IHT to deal with collisions.
    /// 
    /// # Arguments
    /// 
    /// * `num_tilings`—indicates the number of tile indices to be generated (i.e. the length of the returned `Vec`). This value hould be a power of two greater or equal to four times the number of floats according to the original implementation.
    /// * `floats`—a list of floating-point numbers to be tiled
    /// * `ints`—an optional list of integers that will also be tiled; all distinct integers will result in different tilings. In reinforcement learning, discrete actions are often provided here.
    /// 
    /// # Return Value
    /// 
    /// The returned `Vec<usize>` is a vector containing exactly `num_tilings` elements, with each member being an index of a tile encoded by the function. Each member will always be >= 0 and <= size - 1.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use tilecoding::IHT;
    /// // initialize an index-hash-table with size `1024`
    /// let mut iht = IHT::new(1024);
    /// 
    /// // find the indices of tiles for the point (x, y) = (3.6, 7.21) using 8 tilings:
    /// let indices = iht.tiles(8, &[3.6, 7.21], None);
    /// 
    /// // this is the first time we've used the IHT, so we will get the starting tiles:
    /// assert_eq!(indices, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// 
    /// // a nearby point:
    /// let indices = iht.tiles(8, &[3.7, 7.21], None);
    /// 
    /// // differs by one tile:
    /// assert_eq!(indices, vec![0, 1, 2, 8, 4, 5, 6, 7]);
    /// 
    /// // and a point more than one away in any dim
    /// let indices = iht.tiles(8, &[-37.2, 7.0], None);
    /// 
    /// // will have all different tiles
    /// assert_eq!(indices, vec![9, 10, 11, 12, 13, 14, 15, 16]);
    /// ```
    pub fn tiles(&mut self, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<usize> {
        let q_floats = floats
            .iter()
            .map(|&x| (x * num_tilings as f64).floor() as isize)
            .collect::<Vec<isize>>();
        let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let tiling_x2 = tiling as isize * 2;
            let mut coords = Vec::with_capacity(1 + q_floats.len());
            coords.push(tiling as isize);
            let mut b = tiling as isize;
            for q in q_floats.iter() {
                coords.push((q + b) / num_tilings as isize);
                b += tiling_x2;
            }
            if let Some(ints) = ints {
                coords.extend(ints);
            }
            tiles.push(self.get_index(coords));
        }

        tiles
    }

    /// The same as the `tiles` function, except never insert or generate new indices. If an tiling calculate would result in a new tile, return `None` instead
    pub fn tiles_read_only(&mut self, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<Option<usize>> {
        let q_floats = floats
            .iter()
            .map(|&x| (x * num_tilings as f64).floor() as isize)
            .collect::<Vec<isize>>();
        let mut tiles: Vec<Option<usize>> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let tiling_x2 = tiling as isize * 2;
            let mut coords = Vec::with_capacity(1 + q_floats.len());
            coords.push(tiling as isize);
            let mut b = tiling as isize;
            for q in q_floats.iter() {
                coords.push((q + b) / num_tilings as isize);
                b += tiling_x2;
            }
            if let Some(ints) = ints {
                coords.extend(ints);
            }
            tiles.push(self.get_index_read_only(coords));
        }

        tiles
    }
}

/// This function takes a series of floating point and integer values, and encodes them as tile indices using a provided size. This function is generally reserved for when you have extraordinarily large sizes that are too large for the IHT.
/// 
/// # Arguments
/// 
/// * `size`—the upper bounds of all returned indices
/// * `num_tilings`—indicates the number of tile indices to be generated (i.e. the length of the returned `Vec`). This value hould be a power of two greater or equal to four times the number of floats according to the original implementation.
/// * `floats`—a list of floating-point numbers to be tiled
/// * `ints`—an optional list of integers that will also be tiled; all distinct integers will result in different tilings. In reinforcement learning, discrete actions are often provided here.
/// 
/// # Return Value
/// 
/// The returned `Vec<usize>` is a vector containing exactly `num_tilings` elements, with each member being an index of a tile encoded by the function. Each member will always be >= 0 and <= size - 1.
/// 
/// # Examples
/// 
/// ```
/// # use tilecoding::tiles;
/// // find the indices of tiles for the point (x, y) = (3.6, 7.21) using 8 tilings and a maximum size of 1024:
/// let indices = tiles(1024, 8, &[3.6, 7.21], None);
/// 
/// // we get tiles all over the 1024 space as a direct result of the hashing
/// // instead of the more ordered indices provided by an IHT
/// assert_eq!(indices, vec![511, 978, 632, 867, 634, 563, 779, 737]);
/// 
/// // a nearby point:
/// let indices = tiles(1024, 8, &[3.7, 7.21], None);
/// 
/// // differs by one tile:
/// assert_eq!(indices, vec![511, 978, 632, 987, 634, 563, 779, 737]);
/// 
/// // and a point more than one away in any dim
/// let indices = tiles(1024, 8, &[-37.2, 7.0], None);
/// 
/// // will have all different tiles
/// assert_eq!(indices, vec![638, 453, 557, 465, 306, 526, 281, 863]);
/// ```
pub fn tiles(size: usize, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<usize> {
    let q_floats = floats
        .iter()
        .map(|&x| (x * num_tilings as f64).floor() as isize)
        .collect::<Vec<isize>>();
    let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

    for tiling in 0..num_tilings {
        let tiling_x2 = tiling as isize * 2;
        let mut coords = Vec::with_capacity(1 + q_floats.len());
        coords.push(tiling as isize);
        let mut b = tiling as isize;
        for q in q_floats.iter() {
            coords.push((q + b) / num_tilings as isize);
            b += tiling_x2;
        }
        if let Some(ints) = ints {
            coords.extend(ints);
        }
        tiles.push(base_hash(coords) % size);
    }

    tiles
}

#[cfg(test)]
mod tests {
    //extern crate test;

    use super::*;
    //use test::Bencher;

    #[test]
    fn proper_number_of_tiles() {
        let mut iht = IHT::new(32);
        let indices = iht.tiles(8, &[0.0], None);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn same_tiles_for_same_coords() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tiles(8, &[0.0], None);
        let indices_2 = iht.tiles(8, &[0.0], None);
        let indices_3 = iht.tiles(8, &[0.5], None);
        let indices_4 = iht.tiles(8, &[0.5], None);
        let indices_5 = iht.tiles(8, &[1.0], None);
        let indices_6 = iht.tiles(8, &[1.0], None);

        assert_eq!(indices_1, indices_2);
        assert_eq!(indices_3, indices_4);
        assert_eq!(indices_5, indices_6);
    }

    #[test]
    fn different_tiles_for_different_coords() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tiles(8, &[0.0], None);
        let indices_2 = iht.tiles(8, &[0.5], None);
        let indices_3 = iht.tiles(8, &[1.0], None);

        assert_ne!(indices_1, indices_2);
        assert_ne!(indices_2, indices_3);
        assert_ne!(indices_1, indices_3);
    }

    #[test]
    fn can_be_negative() {
        let mut iht = IHT::new(32);
        let indices = iht.tiles(8, &[-10.0], None);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn appropriate_distance() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tiles(4, &[0.0], None);
        let indices_2 = iht.tiles(4, &[0.125], None);
        let indices_3 = iht.tiles(4, &[0.25], None);

        assert_eq!(indices_1, indices_2);
        assert_ne!(indices_1, indices_3);
    }

    #[test]
    fn iht_can_collide() {
        const SIZE: usize = 32;
        let mut iht = IHT::new(SIZE);
        for i in 0..(SIZE * 2) {
            let t = iht.tiles(8, &[i as f64], None);
            assert_eq!(t.len(), 8);
            for j in 0..8 {
                assert!(t[j] < SIZE);
            }
        }
        assert!(iht.full());
    }

    #[test]
    fn read_only_works() {
        let mut iht = IHT::new(32);
        iht.tiles(4, &[0.0], None);
        let indices = iht.tiles_read_only(4, &[32.0], None);
        assert_eq!(indices, vec![None, None, None, None]);
    }

    /*#[bench]
    fn bench_iht_tile_code_small_single_dimension(b: &mut Bencher) {
        let mut iht = IHT::new(32);
        b.iter(|| iht.tiles(8, &[0.0], None));
    }

    #[bench]
    fn bench_iht_tile_code_single_dimension(b: &mut Bencher) {
        let mut iht = IHT::new(2048);
        b.iter(|| iht.tiles(8, &[0.0], None));
    }

    #[bench]
    fn bench_iht_tile_code_small_four_dimensions(b: &mut Bencher) {
        let mut iht = IHT::new(32);
        b.iter(|| iht.tiles(8, &[0.0, 1.0, 2.0, 3.0], None));
    }

    #[bench]
    fn bench_iht_tile_code_four_dimensions(b: &mut Bencher) {
        let mut iht = IHT::new(2048);
        b.iter(|| iht.tiles(8, &[0.0, 1.0, 2.0, 3.0], None));
    }

    #[bench]
    fn bench_non_iht_tile_code_small_single_dimension(b: &mut Bencher) {
        b.iter(|| tiles(32, 8, &[0.0], None));
    }

    #[bench]
    fn bench_non_iht_tile_code_single_dimension(b: &mut Bencher) {
        b.iter(|| tiles(2048, 8, &[0.0], None));
    }

    #[bench]
    fn bench_non_iht_tile_code_small_four_dimensions(b: &mut Bencher) {
        b.iter(|| tiles(32, 8, &[0.0, 1.0, 2.0, 3.0], None));
    }

    #[bench]
    fn bench_non_iht_tile_code_four_dimensions(b: &mut Bencher) {
        b.iter(|| tiles(2048, 8, &[0.0, 1.0, 2.0, 3.0], None));
    }*/
}
