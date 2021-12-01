use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Map Struct - uses a linear backing store to reduce indirection
#[derive(Clone, Debug)]
struct Map {
    width: usize,
    terrain: Vec<bool>,
}

impl Map {
    fn from_file(
        path: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let path = path.as_ref();
        let file = File::open(path)?;

        let mut width = None;
        let mut terrain = Vec::new();

        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if width.is_none() {
                width = Some(line.len());
            }
            terrain.extend(line.as_bytes().iter().map(|&b| b == b'#'));
        }

        Ok(Map {
            width: width.unwrap(),
            terrain,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<&bool> {
        let idx = self.width * y + x % self.width;

        self.terrain.get(idx)
    }

    fn trees_on_path(&self, dx: usize, dy: usize) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut trees = 0;

        while let Some(&pos) = self.get(x, y) {
            if pos {
                trees += 1
            }
            x += dx;
            y += dy;
        }
        trees
    }
}

fn solve1(map: &Map) -> usize {
    map.trees_on_path(3, 1)
}

fn solve2(map: &Map) -> (usize, usize) {
    let rules: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let routes = rules
        .iter()
        .map(|&(x, y)| map.trees_on_path(x, y))
        .collect::<Vec<_>>();
    (routes.iter().product(), *routes.iter().min().unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let map = Map::from_file("inputs/3.txt")?;

    let trees = solve1(&map);
    println!("You only hit {} trees", trees);

    let (prod, min) = solve2(&map);
    println!(
        "The product of all the trees on the possible routes is {} trees²",
        prod
    );
    println!("The best route only involves hititng {} trees", min);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2, Map};

    #[test]
    fn test1() {
        let map = Map::from_file("test.txt").unwrap();
        assert_eq!(solve1(&map), 7);
    }

    #[test]
    fn test2() {
        let map = Map::from_file("test.txt").unwrap();
        assert_eq!(solve2(&map).0, 336);
    }
}
