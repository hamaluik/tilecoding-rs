// benchmarks only available on nightly for now
//#![feature(test)]

use std::collections::HashMap;

// convenience function for hashing a hashable object using the std hashmap's default hasher
fn base_hash<H>(obj: H) -> usize
where
    H: std::hash::Hash,
{
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as usize
}

fn calculate_q_floats(floats: &[f64], num_tilings: usize) -> Vec<isize> {
    floats
        .iter()
        .map(|&x| (x * num_tilings as f64).floor() as isize)
        .collect::<Vec<isize>>()
}

fn calculate_coords(tiling: usize, num_tilings: usize, q_floats: &Vec<isize>, ints: &Option<&[isize]>) -> Vec<isize> {
    let tiling_x2 = tiling as isize * 2;
    let mut coords = Vec::with_capacity(1 + q_floats.len());
    coords.push(tiling as isize);
    let mut b = tiling as isize;
    for q in q_floats.iter() {
        coords.push((q + b) / num_tilings as isize);
        b += tiling_x2;
    }
    if let Some(ints) = ints {
        coords.extend(*ints);
    }

    coords
}

fn calculate_coords_wrap(tiling: usize, num_tilings: usize, q_floats: &Vec<isize>, wrap_widths: &[Option<isize>], ints: &Option<&[isize]>) -> Vec<isize> {
    let tiling_x2 = tiling as isize * 2;
    let mut coords = Vec::with_capacity(1 + q_floats.len());
    coords.push(tiling as isize);
    let mut b = tiling as isize;
    for (q, width) in q_floats.iter().zip(wrap_widths.iter()) {
        let c: isize = (q + b % num_tilings as isize) / num_tilings as isize;
        coords.push(match width {
            Some(w) => c % w,
            None => c,
        });
        b += tiling_x2;
    }
    if let Some(ints) = ints {
        coords.extend(*ints);
    }

    coords
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
    /// # Example
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
        let q_floats = calculate_q_floats(floats, num_tilings);
        let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let coords = calculate_coords(tiling, num_tilings, &q_floats, &ints);
            tiles.push(self.get_index(coords));
        }

        tiles
    }

    /// The same as the `tiles` function, except never insert or generate new indices. If an tiling calculate would result in a new tile, return `None` instead
    pub fn tiles_read_only(&mut self, num_tilings: usize, floats: &[f64], ints: Option<&[isize]>) -> Vec<Option<usize>> {
        let q_floats = calculate_q_floats(floats, num_tilings);
        let mut tiles: Vec<Option<usize>> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let coords = calculate_coords(tiling, num_tilings, &q_floats, &ints);
            tiles.push(self.get_index_read_only(coords));
        }

        tiles
    }

    /// A wrap-around version of `tiles`, described in the [original implementation](http://www.incompleteideas.net/tiles/tiles3.html#Wrap-around_Versions_).
    /// 
    /// # Arguments
    /// 
    /// * `num_tilings`—indicates the number of tile indices to be generated (i.e. the length of the returned `Vec`). This value hould be a power of two greater or equal to four times the number of floats according to the original implementation.
    /// * `floats`—a list of floating-point numbers to be tiled
    /// * `wrap_widths`—a list of optional integer wrapping points.
    /// * `ints`—an optional list of integers that will also be tiled; all distinct integers will result in different tilings. In reinforcement learning, discrete actions are often provided here.
    /// 
    /// # Return Value
    /// 
    /// The returned `Vec<usize>` is a vector containing exactly `num_tilings` elements, with each member being an index of a tile encoded by the function. Each member will always be >= 0 and <= size - 1.
    /// 
    /// # Examples
    /// 
    /// From the [original implementation](http://www.incompleteideas.net/tiles/tiles3.html#Wrap-around_Versions_):
    /// 
    /// > The tilings we have discussed so far stretch out to infinity with no need to specify a range for them. This is cool, but sometimes not what you want. Sometimes you want the variables to wrap-around over some range. For example, you may have an angle variable that goes from 0 to 2π and then should wrap around, that is, you would like generalization to occur between angles just greater than 0 and angles that are nearly 2π. To accommodate this, we provide some alternative, wrap-around versions of the tiling routines.
    /// >
    /// > These versions take an additional input, wrap_widths, which parallels the float structure (array or list), and which specifies for each float the width of the range over which it wraps. If you don't want a float to wrap, it's wrap_width should be [`None`]. The wrap_width is in the same units as the floats, but should be an integer. This can be confusing, so let's do a simple 1D example. Suppose you have one real input, an angle theta. Theta is originally in radians, that is, in [0,2π), which you would like to wrap around at 2π. Remember that these functions all have their tile boundaries at the integers, so if you passed in theta directly there would be tile boundaries at 0, 1, and 2, i.e., just a few tiles over the whole range, which is probably not what you want. So let's say what we want! Suppose we want tilings with 10 tiles over the [0,2π) range. Then we have to scale theta so that it goes from 0 to 10 (instead of 0 to 2π). One would do this by multiplying theta by 10/2π. With the new scaled theta, the wrapping is over [0,10), for a wrap_width of 10.
    /// 
    /// Here is the code for the above case, assuming we want 16 tilings over the original [0, 2π) range with a memory size of 512:
    /// 
    /// ```
    /// # use tilecoding::IHT;
    /// # let theta: f64 = 0.0;
    /// let mut iht = IHT::new(512);
    /// iht.tiles_wrap(
    ///     16,
    ///     &[theta * 10. / (2.0 * std::f64::consts::PI)],
    ///     &[Some(10)],
    ///     None
    /// );
    /// ```
    /// 
    /// > Note that the code would be exactly the same if the original range of theta was [-π,+π]. Specifying the complete range of wrapping (rather than just the width) is not necessary for the same reason as we did not need to give a range at all in the previous routines.
    /// >
    /// > As another example, suppose you wanted to cover the 2π x 3 rectangular area with 16 tilings, each with a width of generalization one-tenth of the space in each dimension, and with wrap-around in the second dimension but not the first. In rust you would do:
    /// 
    /// ```
    /// # use tilecoding::IHT;
    /// # let x: f64 = 0.0;
    /// # let y: f64 = 0.0;
    /// let mut iht = IHT::new(512);
    /// iht.tiles_wrap(
    ///     16,
    ///     &[x / (3. * 0.1), y / (2.0 * std::f64::consts::PI * 0.1)],
    ///     &[None, Some(10)],
    ///     None
    /// );
    /// ```
    pub fn tiles_wrap(&mut self, num_tilings: usize, floats: &[f64], wrap_widths: &[Option<isize>], ints: Option<&[isize]>) -> Vec<usize> {
        let q_floats = calculate_q_floats(floats, num_tilings);
        let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let coords = calculate_coords_wrap(tiling, num_tilings, &q_floats, wrap_widths, &ints);
            tiles.push(self.get_index(coords));
        }

        tiles
    }

    /// The read-only version of `tiles_wrap`
    pub fn tiles_wrap_read_only(&mut self, num_tilings: usize, floats: &[f64], wrap_widths: &[Option<isize>], ints: Option<&[isize]>) -> Vec<Option<usize>> {
        let q_floats = calculate_q_floats(floats, num_tilings);
        let mut tiles: Vec<Option<usize>> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

        for tiling in 0..num_tilings {
            let coords = calculate_coords_wrap(tiling, num_tilings, &q_floats, wrap_widths, &ints);
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
/// # Example
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
    let q_floats = calculate_q_floats(floats, num_tilings);
    let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

    for tiling in 0..num_tilings {
        let coords = calculate_coords(tiling, num_tilings, &q_floats, &ints);
        tiles.push(base_hash(coords) % size);
    }

    tiles
}

/// A wrap-around version of `tiles`, described in the [original implementation](http://www.incompleteideas.net/tiles/tiles3.html#Wrap-around_Versions_).
/// 
/// # Arguments
/// 
/// * `size`—the upper bounds of all returned indices
/// * `num_tilings`—indicates the number of tile indices to be generated (i.e. the length of the returned `Vec`). This value hould be a power of two greater or equal to four times the number of floats according to the original implementation.
/// * `floats`—a list of floating-point numbers to be tiled
/// * `wrap_widths`—a list of optional integer wrapping points.
/// * `ints`—an optional list of integers that will also be tiled; all distinct integers will result in different tilings. In reinforcement learning, discrete actions are often provided here.
/// 
/// # Return Value
/// 
/// The returned `Vec<usize>` is a vector containing exactly `num_tilings` elements, with each member being an index of a tile encoded by the function. Each member will always be >= 0 and <= size - 1.
/// 
/// # Examples
/// 
/// From the [original implementation](http://www.incompleteideas.net/tiles/tiles3.html#Wrap-around_Versions_):
/// 
/// > The tilings we have discussed so far stretch out to infinity with no need to specify a range for them. This is cool, but sometimes not what you want. Sometimes you want the variables to wrap-around over some range. For example, you may have an angle variable that goes from 0 to 2π and then should wrap around, that is, you would like generalization to occur between angles just greater than 0 and angles that are nearly 2π. To accommodate this, we provide some alternative, wrap-around versions of the tiling routines.
/// >
/// > These versions take an additional input, wrap_widths, which parallels the float structure (array or list), and which specifies for each float the width of the range over which it wraps. If you don't want a float to wrap, it's wrap_width should be [`None`]. The wrap_width is in the same units as the floats, but should be an integer. This can be confusing, so let's do a simple 1D example. Suppose you have one real input, an angle theta. Theta is originally in radians, that is, in [0,2π), which you would like to wrap around at 2π. Remember that these functions all have their tile boundaries at the integers, so if you passed in theta directly there would be tile boundaries at 0, 1, and 2, i.e., just a few tiles over the whole range, which is probably not what you want. So let's say what we want! Suppose we want tilings with 10 tiles over the [0,2π) range. Then we have to scale theta so that it goes from 0 to 10 (instead of 0 to 2π). One would do this by multiplying theta by 10/2π. With the new scaled theta, the wrapping is over [0,10), for a wrap_width of 10.
/// 
/// Here is the code for the above case, assuming we want 16 tilings over the original [0, 2π) range with a memory size of 512:
/// 
/// ```
/// # use tilecoding::tiles_wrap;
/// # let theta: f64 = 0.0;
/// tiles_wrap(
///     512,
///     16,
///     &[theta * 10. / (2.0 * std::f64::consts::PI)],
///     &[Some(10)],
///     None
/// );
/// ```
/// 
/// > Note that the code would be exactly the same if the original range of theta was [-π,+π]. Specifying the complete range of wrapping (rather than just the width) is not necessary for the same reason as we did not need to give a range at all in the previous routines.
/// >
/// > As another example, suppose you wanted to cover the 2π x 3 rectangular area with 16 tilings, each with a width of generalization one-tenth of the space in each dimension, and with wrap-around in the second dimension but not the first. In rust you would do:
/// 
/// ```
/// # use tilecoding::tiles_wrap;
/// # let x: f64 = 0.0;
/// # let y: f64 = 0.0;
/// tiles_wrap(
///     512,
///     16,
///     &[x / (3. * 0.1), y / (2.0 * std::f64::consts::PI * 0.1)],
///     &[None, Some(10)],
///     None
/// );
/// ```
pub fn tiles_wrap(size: usize, num_tilings: usize, floats: &[f64], wrap_widths: &[Option<isize>], ints: Option<&[isize]>) -> Vec<usize> {
    let q_floats = calculate_q_floats(floats, num_tilings);
    let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings + ints.unwrap_or(&[]).len());

    for tiling in 0..num_tilings {
        let coords = calculate_coords_wrap(tiling, num_tilings, &q_floats, wrap_widths, &ints);
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

    #[test]
    fn wrapping_works() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tiles_wrap(4, &[0.0], &[Some(10)], None);
        let indices_2 = iht.tiles_wrap(4, &[10.0], &[Some(10)], None);

        assert_eq!(indices_1, indices_2);
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
