#![feature(test)]

pub struct TileCoords {
    size: usize,
    num_tilings: usize,
    rand_sequence: [isize; 2048],
}

impl TileCoords {
    pub fn new(size: usize, num_tilings: usize) -> TileCoords {
        TileCoords {
            size,
            num_tilings,
            rand_sequence: {
                let mut rs: [isize; 2048] = [0; 2048];
                for k in 0..2048 {
                    for _ in 0..std::mem::size_of::<isize>() {
                        rs[k] = (rs[k] << 8) | (rand::random::<isize>() & 0xff);
                    }
                }
                rs
            },
        }
    }

    fn hash_unh(&self, coordinates: &[isize], m: isize, increment: isize) -> usize {
        let mut index: isize;
        let mut sum: isize = 0;

        for i in 0..coordinates.len() {
            index = coordinates[i];
            index += increment * i as isize;
            index = index & 2047;
            while index < 0 {
                index += 2048;
            }
            sum = sum.wrapping_add(self.rand_sequence[index as usize]);
        }

        index = sum as isize % m;
        while index < 0 {
            index += m;
        }

        index as usize
    }

    pub fn tile(&self, data_point: &[f32]) -> Vec<usize> {
        let mut base: Vec<isize> = vec![0; data_point.len()];
        let mut tiles: Vec<usize> = Vec::with_capacity(self.num_tilings);
        let mut coordinates: Vec<isize> = vec![0; data_point.len() + 1];

        // quantize the state to integers (henceforth, tile widths == self.num_tilings)
        let qstate = data_point.iter().map(|&x| (x * self.num_tilings as f32).floor() as isize).collect::<Vec<isize>>();

        // compute the tile numbers
        for tiling in 0..self.num_tilings {
            // loop over each relevant dimension
            for i in 0..data_point.len() {
                // find coordinates of activated tile in tiling space
                coordinates[i] =
                    if qstate[i] >= base[i] {
                        qstate[i] - ((qstate[i] - base[i]) % self.num_tilings as isize)
                    }
                    else {
                        qstate[i] + 1 + ((base[i] - qstate[i] - 1) % self.num_tilings as isize) - self.num_tilings as isize
                    };
                
                // compute displacement of next tiling in quantized space
                base[i] += 1 + (2 * i as isize);
            }
            // add additional indices for tiling and hashing_set so they hash differently
            coordinates[data_point.len()] = tiling as isize;

            // calculate the tile
            tiles.push(self.hash_unh(&coordinates, self.size as isize, 449));
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
        let iht = TileCoords::new(32, 8);
        let indices = iht.tile(&[0.0]);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn same_tiles_for_same_coords() {
        let iht = TileCoords::new(32, 8);
        let indices_1 = iht.tile(&[0.0]);
        let indices_2 = iht.tile(&[0.0]);
        let indices_3 = iht.tile(&[0.5]);
        let indices_4 = iht.tile(&[0.5]);
        let indices_5 = iht.tile(&[1.0]);
        let indices_6 = iht.tile(&[1.0]);

        assert_eq!(indices_1, indices_2);
        assert_eq!(indices_3, indices_4);
        assert_eq!(indices_5, indices_6);
    }

    #[test]
    fn different_tiles_for_different_coords() {
        let iht = TileCoords::new(32, 8);
        let indices_1 = iht.tile(&[0.0]);
        let indices_2 = iht.tile(&[0.5]);
        let indices_3 = iht.tile(&[1.0]);

        assert_ne!(indices_1, indices_2);
        assert_ne!(indices_2, indices_3);
        assert_ne!(indices_1, indices_3);
    }

    #[test]
    fn can_be_negative() {
        let iht = TileCoords::new(32, 8);
        let indices = iht.tile(&[-10.0]);
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn appropriate_distance() {
        let iht = TileCoords::new(32, 4);
        let indices_1 = iht.tile(&[0.0]);
        let indices_2 = iht.tile(&[0.125]);
        let indices_3 = iht.tile(&[0.25]);

        assert_eq!(indices_1, indices_2);
        assert_ne!(indices_1, indices_3);
    }

    #[bench]
    fn bench_tile_code_small_single_dimension(b: &mut Bencher) {
        let iht = TileCoords::new(32, 8);
        b.iter(|| iht.tile(&[0.0]));
    }

    #[bench]
    fn bench_tile_code_single_dimension(b: &mut Bencher) {
        let iht = TileCoords::new(2048, 8);
        b.iter(|| iht.tile(&[0.0]));
    }

    #[bench]
    fn bench_tile_code_small_four_dimensions(b: &mut Bencher) {
        let iht = TileCoords::new(32, 8);
        b.iter(|| iht.tile(&[0.0, 1.0, 2.0, 3.0]));
    }

    #[bench]
    fn bench_tile_code_four_dimensions(b: &mut Bencher) {
        let iht = TileCoords::new(2048, 8);
        b.iter(|| iht.tile(&[0.0, 1.0, 2.0, 3.0]));
    }
}
