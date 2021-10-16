use aoc::*;
use std::collections::HashMap;

fn main() {
    let coords: Vec<_> = parser::lines::<Coord<usize>>().collect();
    let max_x = coords.iter().max_by_key(|c| c.x).unwrap().x;
    let max_y = coords.iter().max_by_key(|c| c.y).unwrap().y;

    // create a grid with enough space to fit all coordinates
    let grid = Grid::with_dimension(max_x + 1, max_y + 1);
    let mut grid = grid.map(|el| (el, usize::MAX));

    for (id, coord) in coords.iter().enumerate() {
        let id = id + 1;

        grid.enumerate_mut().for_each(|(c, (i, d))| {
            let distance = c.distance_from(coord);
            if distance < *d {
                *i = id;
                *d = distance;
            } else if distance == *d {
                *i = 0;
            }
        });
    }

    let grid = grid.map(|(id, _distance)| id);
    let mut map = grid.iter().fold(HashMap::new(), |mut map, id| {
        *map.entry(id).or_insert(0) += 1;
        map
    });
    grid.borders().for_each(|id| drop(map.remove(id)));
    let largest_area = map.values().max().unwrap();

    answer!(
        "The size of the largest area that isn't infinite is {}.",
        largest_area
    );
}
