use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

// A range mapping; defines a range from src..=(src+len) to dst..=(dst+len)
#[derive(Debug)]
pub struct RangeMap {
    pub src: u64,
    pub dst: u64,
    pub len: u64,
}

impl RangeMap {
    // If x is in the source range, map to the destination
    pub fn apply(&self, x: u64) -> Option<u64> {
        if x < self.src || x >= self.src + self.len {
            None
        } else {
            Some(self.dst + x - self.src)
        }
    }

    // Apply over an input range
    // Returns three optional ranges:
    // 1. The portion of the original range below self's range
    // 2. The portion of the original range overlapping self's range mapped to destination
    // 3. The portion of the original range above self's range
    #[allow(clippy::type_complexity)]
    pub fn apply_range(
        &self,
        input: RangeInclusive<u64>,
    ) -> (
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
    ) {
        let (input_start, input_end) = input.clone().into_inner();
        let src_end = self.src + self.len - 1;

        let below = if input_start < self.src {
            Some(input_start..=self.src.saturating_sub(1).min(input_end))
        } else {
            None
        };

        let overlap = if input_end >= self.src && input_start <= src_end {
            let overlap_start = input_start.max(self.src);
            let overlap_end = input_end.min(src_end);
            Some((self.dst + overlap_start - self.src)..=(self.dst + overlap_end - self.src))
        } else {
            None
        };

        let above = if input_end > src_end {
            Some(src_end.saturating_add(1).max(input_start)..=input_end)
        } else {
            None
        };

        (below, overlap, above)
    }
}

#[derive(Debug)]
pub struct CategoryMap {
    pub src_cat: Category,
    pub dst_cat: Category,
    pub range_maps: Vec<RangeMap>,
}

impl CategoryMap {
    pub fn apply(&self, x: u64) -> u64 {
        self.range_maps
            .iter()
            .find_map(|range_map| range_map.apply(x))
            .unwrap_or(x)
    }

    pub fn apply_range(&self, input: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut ranges = vec![input.clone()];
        let mut result = vec![];

        for range_map in self.range_maps.iter() {
            let mut unchanged = vec![];

            // Mapped ranges are ready to return
            // Anything else passes to the next range map
            for range in ranges.iter() {
                let (below, overlap, above) = range_map.apply_range(range.clone());
                if let Some(below) = below {
                    unchanged.push(below);
                }
                if let Some(overlap) = overlap {
                    result.push(overlap);
                }
                if let Some(above) = above {
                    unchanged.push(above);
                }
            }

            ranges.clear();
            ranges.append(&mut unchanged);
        }

        // Any unchanged ranges after all maps are returned
        result.append(&mut ranges);

        result
    }
}

#[derive(Debug)]
pub struct Simulation {
    pub seeds: Vec<u64>,
    pub category_maps: Vec<CategoryMap>,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_range_map_apply() {
        let range_map = super::RangeMap {
            src: 5,
            dst: 10,
            len: 10,
        };
        assert_eq!(range_map.apply(4), None);
        assert_eq!(range_map.apply(5), Some(10));
        assert_eq!(range_map.apply(6), Some(11));
        assert_eq!(range_map.apply(14), Some(19));
        assert_eq!(range_map.apply(15), None);
    }

    #[test]
    fn test_category_map_apply() {
        let category_map = super::CategoryMap {
            src_cat: super::Category::Seed,
            dst_cat: super::Category::Soil,
            range_maps: vec![
                super::RangeMap {
                    src: 50,
                    dst: 98,
                    len: 2,
                },
                super::RangeMap {
                    src: 52,
                    dst: 50,
                    len: 48,
                },
            ],
        };
        assert_eq!(category_map.apply(79), 77);
    }

    #[test]
    fn test_range_map_range_apply() {
        let range_map = super::RangeMap {
            src: 5,
            dst: 10,
            len: 10,
        };
        // all low
        assert_eq!(range_map.apply_range(1..=4), (Some(1..=4), None, None));
        // low and mid
        assert_eq!(
            range_map.apply_range(1..=6),
            (Some(1..=4), Some(10..=11), None)
        );
        // all mid
        assert_eq!(range_map.apply_range(6..=14), (None, Some(11..=19), None));
        // mid and high
        assert_eq!(
            range_map.apply_range(6..=16),
            (None, Some(11..=19), Some(15..=16))
        );
        // all three
        assert_eq!(
            range_map.apply_range(1..=15),
            (Some(1..=4), Some(10..=19), Some(15..=15))
        );
    }
}
