use aoc::*;

fn main() {
    let coords: Vec<_> = parser::lines::<Coord<usize>>().collect();
    let max_x = coords.iter().max_by_key(|c| c.x).unwrap().x;
    let max_y = coords.iter().max_by_key(|c| c.y).unwrap().y;

    let region_size = Coord::at(0, 0)
        .to(Coord::at(max_x, max_y))
        .unwrap()
        .filter(|coord| coords.iter().map(|c| coord.distance_from(c)).sum::<usize>() < 10000)
        .count();

    answer!("The size of the region containing all locations which have a total distance to all given coordinates of less than 10000 is {}.", region_size);
}
