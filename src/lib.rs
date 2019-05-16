#![feature(test)]

use std::collections::HashMap;

fn basehash<H>(obj: H) -> usize
    where H: std::hash::Hash {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as usize
}

pub struct IHT {
    size: usize,
    overfull_count: usize,
    dictionary: HashMap<Vec<isize>, usize>,
}

impl IHT {
    pub fn new(size: usize) -> IHT {
        IHT {
            size,
            overfull_count: 0,
            dictionary: HashMap::with_capacity(size),
        }
    }

    fn get_index(&mut self, obj: Vec<isize>) -> usize {
        let count = self.dictionary.len();
        use std::collections::hash_map::Entry;
        match self.dictionary.entry(obj) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                if count >= self.size {
                    self.overfull_count += 1;
                    basehash(v.into_key())
                }
                else {
                    *v.insert(count)
                }
            }
        }
    }

    pub fn tile(&mut self, num_tilings: usize, data_point: &[f64]) -> Vec<usize> {
        let q_floats = data_point.iter().map(|&x| (x * num_tilings as f64).floor() as isize).collect::<Vec<isize>>();
        let mut tiles: Vec<usize> = Vec::with_capacity(num_tilings);

        for tiling in 0..num_tilings {
            let tiling_x2 = tiling as isize * 2;
            let mut coords = Vec::with_capacity(1 + q_floats.len());
            coords.push(tiling as isize);
            let mut b = tiling as isize;
            for q in q_floats.iter() {
                coords.push((q + b) / num_tilings as isize);
                b += tiling_x2;
            }

            tiles.push(self.get_index(coords));
        }

        tiles
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn proper_number_of_tiles() {
        let mut iht = IHT::new(32);
        let indices = iht.tile(8, &[0.0]);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn same_tiles_for_same_coords() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tile(8, &[0.0]);
        let indices_2 = iht.tile(8, &[0.0]);
        let indices_3 = iht.tile(8, &[0.5]);
        let indices_4 = iht.tile(8, &[0.5]);
        let indices_5 = iht.tile(8, &[1.0]);
        let indices_6 = iht.tile(8, &[1.0]);

        assert_eq!(indices_1, indices_2);
        assert_eq!(indices_3, indices_4);
        assert_eq!(indices_5, indices_6);
    }

    #[test]
    fn different_tiles_for_different_coords() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tile(8, &[0.0]);
        let indices_2 = iht.tile(8, &[0.5]);
        let indices_3 = iht.tile(8, &[1.0]);

        assert_ne!(indices_1, indices_2);
        assert_ne!(indices_2, indices_3);
        assert_ne!(indices_1, indices_3);
    }

    #[test]
    fn can_be_negative() {
        let mut iht = IHT::new(32);
        let indices = iht.tile(8, &[-10.0]);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn appropriate_distance() {
        let mut iht = IHT::new(32);
        let indices_1 = iht.tile(4, &[0.0]);
        let indices_2 = iht.tile(4, &[0.125]);
        let indices_3 = iht.tile(4, &[0.25]);

        assert_eq!(indices_1, indices_2);
        assert_ne!(indices_1, indices_3);
    }

    #[bench]
    fn bench_tile_code_small_single_dimension(b: &mut Bencher) {
        let mut iht = IHT::new(32);
        b.iter(|| iht.tile(8, &[0.0]));
    }

    #[bench]
    fn bench_tile_code_single_dimension(b: &mut Bencher) {
        let mut iht = IHT::new(2048);
        b.iter(|| iht.tile(8, &[0.0]));
    }

    #[bench]
    fn bench_tile_code_small_four_dimensions(b: &mut Bencher) {
        let mut iht = IHT::new(32);
        b.iter(|| iht.tile(8, &[0.0, 1.0, 2.0, 3.0]));
    }

    #[bench]
    fn bench_tile_code_four_dimensions(b: &mut Bencher) {
        let mut iht = IHT::new(2048);
        b.iter(|| iht.tile(8, &[0.0, 1.0, 2.0, 3.0]));
    }
}
