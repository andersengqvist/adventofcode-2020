use crate::day;
use crate::file;
use std::collections::HashMap;
use std::cmp::max;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day20 {

}

impl day::Day for Day20 {

    fn puzzle1(&self) {
        println!("Day 20, puzzle 1");

        let result = get_result_1(&file::lines("res/day20_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 20, puzzle 2");

        let result = get_result_2(&file::lines("res/day20_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> u128 {
    let tiles = create_image_tiles(&input);
    let corners = find_corners(&tiles);
    corners.iter()
        .map(|&v| v as u128)
        .fold(
            1,
            |a, s| a * s
        )
}

fn get_result_2(input: &[String]) -> usize {
    let tiles = create_image_tiles(&input);
    let mut image = build_image(&tiles);
    find_monsters(&mut image);
    image.calculate_water_roughness()
}

fn create_image_tiles(input: &[String]) -> Vec<ImageTile> {
    let mut result = Vec::new();

    let mut begin = 0;
    for i in 1..input.len() {
        let line = &input[i];
        if line.is_empty() {
            // Beginning of a new tile
            result.push(create_image_tile(&input[begin..i]));
            begin = i + 1;
        }
    }
    result.push(create_image_tile(&input[begin..]));

    result
}

lazy_static! {
    static ref RE: Regex = Regex::new("Tile (\\d+):").unwrap();
}

fn create_image_tile(input: &[String]) -> ImageTile {
    let cap = RE.captures(&input[0]).unwrap();
    let id: u32 = cap[1].parse::<u32>().expect("Could not parse tile id");
    let mut pixels: [u16; 10] = [0; 10];

    for row in 1..input.len() {
        let mut r: u16 = 0;
        let row_pixels = input[row].chars().collect::<Vec<char>>();
        for col in 0..row_pixels.len() {
            let pixel = if row_pixels[col] == '#' { 1 } else { 0 };
            r |= (pixel << row_pixels.len() - 1 - col) as u16;
        }
        pixels[row-1] = r;
    }

    ImageTile::new(id, pixels)
}

fn find_corners(tiles: &[ImageTile]) -> Vec<u32> {
    let edge_map = map_edge_to_image_id(tiles);
    let single_edges: HashMap<u32, u32> = map_image_id_to_single_edges(&edge_map);
    find_corners_in_single_edges(&single_edges)
}

/// Maps the edge signature to image ids.
/// The values should be length 1 if a border edge, 2 otherwise
/// Assumes there are no more edges that share the same signature
fn map_edge_to_image_id(tiles: &[ImageTile]) -> HashMap<u16, Vec<u32>> {
    let mut edge_map: HashMap<u16, Vec<u32>> = HashMap::new();

    for tile in tiles {
        add_edge(&mut edge_map, tile.get_top_border_id(), tile.id);
        add_edge(&mut edge_map, tile.get_bottom_border_id(), tile.id);
        add_edge(&mut edge_map, tile.get_right_border_id(), tile.id);
        add_edge(&mut edge_map, tile.get_left_border_id(), tile.id);
    }
    edge_map
}

fn add_edge(edge_map: &mut HashMap<u16, Vec<u32>>, edge: (u16, u16), id: u32) {
    let vmax = max(edge.0, edge.1);
    let vec = edge_map.get_mut(&vmax);
    if vec.is_some() {
        vec.unwrap().push(id);
    } else {
        edge_map.insert(vmax, vec![id]);
    }

}

fn get_num_edge_neighbours(edge_map: &HashMap<u16, Vec<u32>>, edge: (u16, u16)) -> usize {
    let vmax = max(edge.0, edge.1);
    edge_map.get(&vmax).unwrap().len()
}

fn get_edge_neighbour(edge_map: &HashMap<u16, Vec<u32>>, edge: (u16, u16), neighbour_to_id: u32) -> Option<u32> {
    let vmax = max(edge.0, edge.1);
    let neighbours = edge_map.get(&vmax).unwrap();
    for &n in neighbours {
        if n != neighbour_to_id {
            return Option::Some(n);
        }
    }
    Option::None
}

/// Maps image id to num single edges of that image
fn map_image_id_to_single_edges(edge_map: &HashMap<u16, Vec<u32>>) -> HashMap<u32, u32> {
    let mut single_edges: HashMap<u32, u32> = HashMap::new();
    for edges in edge_map.values() {
        if edges.len() == 1 {
            let tile_id = edges[0];
            *single_edges.entry(tile_id).or_insert(0) += 1;
        } else if edges.len() != 2 {
            panic!("{} edges has the same signature", edges.len());
        }
    }
    single_edges
}

fn find_corners_in_single_edges(single_edges:&HashMap<u32, u32>) -> Vec<u32> {
    single_edges
        .iter()
        .filter_map(|(&id, &num)| {
            if num == 2 {
                // Two single edges, i.e. a corner
                Option::Some(id)
            } else if num == 0 || num == 1 {
                // A side or middle piece
                Option::None
            } else {
                panic!("Image {} has {} single edges", id, num);
            }
        })
        .collect::<Vec<u32>>()
}

fn build_image(tiles: &[ImageTile]) -> Image {
    let tile_map: HashMap<u32, &ImageTile> = tiles.iter().map(|tile| (tile.id, tile)).collect();
    let edge_map = map_edge_to_image_id(tiles);
    let single_edges: HashMap<u32, u32> = map_image_id_to_single_edges(&edge_map);
    let corners = find_corners_in_single_edges(&single_edges);

    let image_size = (tiles.len() as f64).sqrt() as usize;
    let mut image = Image::new(image_size * 8);
    let mut tile_pos: HashMap<(usize, usize), ImageTile> = HashMap::new();

    for t_row in 0..image_size {
        for t_col in 0..image_size {
            if t_row == 0 && t_col == 0 {
                let mut top_left: ImageTile = *tile_map.get(&corners[0]).unwrap().clone();
                while get_num_edge_neighbours(&edge_map, top_left.get_left_border_id()) != 1
                    || get_num_edge_neighbours(&edge_map, top_left.get_top_border_id()) != 1 {
                    top_left = top_left.rotate_right();
                }
                fill_image(&mut image, &top_left, t_row, t_col);
                tile_pos.insert((t_row, t_col), top_left);
            } else if t_col == 0 {
                let tile_above = tile_pos.get(&(t_row-1, t_col)).unwrap();
                let edge_id = tile_above.get_bottom_border_id();
                let tile_id = get_edge_neighbour(&edge_map, edge_id, tile_above.id).unwrap();
                let mut tile = *tile_map.get(&tile_id).unwrap().clone();
                while !edges_matches(edge_id, tile.get_top_border_id()) {
                    tile = tile.rotate_right();
                }
                if tile.get_top_border_id() != edge_id {
                    tile = tile.flip_vertical();
                }
                if !edges_matches(edge_id, tile.get_top_border_id()) {
                    panic!("Edges does not match anymore after a flip!!");
                }
                fill_image(&mut image, &tile, t_row, t_col);
                tile_pos.insert((t_row, t_col), tile);
            } else {
                let tile_left = tile_pos.get(&(t_row, t_col-1)).unwrap();
                let edge_id = tile_left.get_right_border_id();
                let tile_id = get_edge_neighbour(&edge_map, edge_id, tile_left.id).unwrap();
                let mut tile = *tile_map.get(&tile_id).unwrap().clone();
                while !edges_matches(edge_id, tile.get_left_border_id()) {
                    tile = tile.rotate_right();
                }
                if tile.get_left_border_id() != edge_id {
                    tile = tile.flip_horizontal();
                }
                if !edges_matches(edge_id, tile.get_left_border_id()) {
                    panic!("Edges does not match anymore after a flip!!");
                }
                fill_image(&mut image, &tile, t_row, t_col);
                tile_pos.insert((t_row, t_col), tile);
            }
        }
    }

    image
}

fn fill_image(image: &mut Image, tile: &ImageTile, tile_row: usize, tile_col: usize) {
    for r in 1..9 {
        for c in 1..9 {
            let p = tile.get_pixel(r, c);
            if p != 0 {
                image.set_pixel(tile_row * 8 + r as usize - 1, tile_col * 8 + c as usize - 1, Pixel::POUND);
            }
        }
    }
}

fn edges_matches(edge1: (u16, u16), edge2: (u16, u16)) -> bool {
    (edge1.0 == edge2.0 && edge1.1 == edge2.1) || (edge1.0 == edge2.1 && edge1.1 == edge2.0)
}

fn find_monsters(image: &mut Image) {
    let monster_pixels = vec![
        (0,18),
        (1,0),(1,5),(1,6),(1,11),(1,12),(1,17),(1,18),(1,19),
        (2,1),(2,4),(2,7),(2,10),(2,13),(2,16)
    ];
    let image_mask1 = ImageMask::new(monster_pixels);
    let image_mask2 = image_mask1.rotate_right();
    let image_mask3 = image_mask2.rotate_right();
    let image_mask4 = image_mask3.rotate_right();
    let image_mask_r1 = image_mask1.flip_vertically();
    let image_mask_r2 = image_mask_r1.rotate_right();
    let image_mask_r3 = image_mask_r2.rotate_right();
    let image_mask_r4 = image_mask_r3.rotate_right();

    let monsters: Vec<ImageMask> = vec![
        image_mask1,
        image_mask2,
        image_mask3,
        image_mask4,
        image_mask_r1,
        image_mask_r2,
        image_mask_r3,
        image_mask_r4
    ];

    for mm in &monsters {
        if paint_monsters(image, mm) > 0 {
            break;
        }
    }
}

fn paint_monsters(image: &mut Image, monster: &ImageMask) -> usize {
    let mut monsters = 0;
    for row in 0..image.rows()-monster.rows {
        for col in 0..image.cols()-monster.cols {
            if image.matches_mask(row, col, monster) {
                image.paint_monster(row, col, monster);
                monsters += 1;
            }
        }
    }
    monsters
}

struct ImageMask {
    mask: Vec<(usize, usize)>,
    rows: usize,
    cols: usize
}

impl ImageMask {
    fn new(mask: Vec<(usize, usize)>) -> ImageMask {
        let (rows, cols) = mask
            .iter()
            .fold(
                (0usize, 0usize),
                |(r0, c0), (r1, c1)| {
                    (max(r0, *r1), max(c0, *c1))
                }
            );
        ImageMask { mask, rows: rows+1, cols: cols+1 }
    }

    fn rotate_right(&self) -> ImageMask {
        let new_mask =
            self.mask.iter().map(|&(r,c)| (c, self.rows-r-1)).collect();
        ImageMask::new(new_mask)
    }

    fn flip_vertically(&self) -> ImageMask {
        let new_mask =
            self.mask.iter().map(|&(r,c)| (r, self.cols-c-1)).collect();
        ImageMask::new(new_mask)
    }

}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Pixel {
    DOT,
    POUND,
    MONSTER
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Image {
    pixels: Vec<Vec<Pixel>>
}

impl Image {
    fn new(size: usize) -> Image {
        let pixels = vec![vec![Pixel::DOT; size]; size];
        Image { pixels }
    }

    fn get_pixel(&self, row: usize, col: usize) -> Pixel {
        self.pixels[row][col]
    }

    fn set_pixel(&mut self, row: usize, col: usize, pixel: Pixel) {
        self.pixels[row][col] = pixel;
    }

    fn matches_mask(&self, row: usize, col: usize, monster: &ImageMask) -> bool {
        for t in &monster.mask {
            let pixel = self.get_pixel(row + t.0, col + t.1);
            if pixel != Pixel::POUND {
                return false;
            }
        }
        true
    }

    fn paint_monster(&mut self, row: usize, col: usize, monster: &ImageMask) {
        for t in &monster.mask {
            self.set_pixel(row + t.0, col + t.1, Pixel::MONSTER);
        }
    }

    fn calculate_water_roughness(&self) -> usize {
        let mut res = 0;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pixel = self.get_pixel(row, col);
                if pixel == Pixel::POUND {
                    res += 1;
                }
            }
        }
        res
    }

    fn cols(&self) -> usize {
        self.pixels[0].len()
    }

    fn rows(&self) -> usize {
        self.pixels.len()
    }

}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct ImageTile {
    id: u32,
    pixels: [u16; 10]
}

impl ImageTile {
    fn new(id: u32, pixels: [u16; 10]) -> ImageTile {
        ImageTile { id, pixels }
    }

    fn get_pixel(&self, row: u8, col: u8) -> u8 {
        ((self.pixels[row as usize] >> ((9 - col) as u16)) & 1) as u8
    }

    fn cols(&self) -> u8 {
        10
    }

    fn rows(&self) -> u8 {
        10
    }

    fn get_top_border_id(&self) -> (u16, u16) {
        let mut id: u16 = 0;
        let mut rev_id: u16 = 0;
        let row = 0;
        for col in 0..self.cols() {
            let pixel = self.get_pixel(row, col) as u16;
            id |= pixel << col as u16;
            rev_id |= pixel << (self.cols() - 1 - col) as u16;
        }
        (id, rev_id)
    }

    fn get_bottom_border_id(&self) -> (u16, u16) {
        let mut id: u16 = 0;
        let mut rev_id: u16 = 0;
        let row = self.rows() - 1;
        for col in 0..self.cols() {
            let pixel = self.get_pixel(row, col) as u16;
            id |= pixel << col as u16;
            rev_id |= pixel << (self.cols() - 1 - col) as u16;
        }
        (id, rev_id)
    }

    fn get_left_border_id(&self) -> (u16, u16) {
        let mut id: u16 = 0;
        let mut rev_id: u16 = 0;
        let col = 0;
        for row in 0..self.rows() {
            let pixel = self.get_pixel(row, col) as u16;
            id |= pixel << row as u16;
            rev_id |= pixel << (self.rows() - 1 - row) as u16;
        }
        (id, rev_id)
    }

    fn get_right_border_id(&self) -> (u16, u16) {
        let mut id: u16 = 0;
        let mut rev_id: u16 = 0;
        let col = self.cols() - 1;
        for row in 0..self.rows() {
            let pixel = self.get_pixel(row, col) as u16;
            id |= pixel << row as u16;
            rev_id |= pixel << (self.rows() - 1 - row) as u16;
        }
        (id, rev_id)
    }

    fn rotate_right(&self) -> Self {
        let mut pixels: [u16; 10] = [0; 10];

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pixel = self.get_pixel(row, col) as u16;
                if pixel == 1 {
                    pixels[col as usize] |= pixel << row as u16;
                }
            }
        }
        ImageTile { id: self.id, pixels }
    }

    fn flip_horizontal(&self) -> Self {
        let mut pixels: [u16; 10] = [0; 10];

        for col in 0..self.cols() {
            pixels[col as usize] = self.pixels[(self.cols() - 1 - col) as usize];
        }

        ImageTile { id: self.id, pixels }
    }

    fn flip_vertical(&self) -> Self {
        let mut pixels: [u16; 10] = [0; 10];

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pixel = self.get_pixel(row, col) as u16;
                if pixel == 1 {
                    pixels[row as usize] |= pixel << col as u16;
                }
            }
        }

        ImageTile { id: self.id, pixels }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "Tile 2311:".to_string(),
            "..##.#..#.".to_string(),
            "##..#.....".to_string(),
            "#...##..#.".to_string(),
            "####.#...#".to_string(),
            "##.##.###.".to_string(),
            "##...#.###".to_string(),
            ".#.#.#..##".to_string(),
            "..#....#..".to_string(),
            "###...#.#.".to_string(),
            "..###..###".to_string(),
            "".to_string(),
            "Tile 1951:".to_string(),
            "#.##...##.".to_string(),
            "#.####...#".to_string(),
            ".....#..##".to_string(),
            "#...######".to_string(),
            ".##.#....#".to_string(),
            ".###.#####".to_string(),
            "###.##.##.".to_string(),
            ".###....#.".to_string(),
            "..#.#..#.#".to_string(),
            "#...##.#..".to_string(),
            "".to_string(),
            "Tile 1171:".to_string(),
            "####...##.".to_string(),
            "#..##.#..#".to_string(),
            "##.#..#.#.".to_string(),
            ".###.####.".to_string(),
            "..###.####".to_string(),
            ".##....##.".to_string(),
            ".#...####.".to_string(),
            "#.##.####.".to_string(),
            "####..#...".to_string(),
            ".....##...".to_string(),
            "".to_string(),
            "Tile 1427:".to_string(),
            "###.##.#..".to_string(),
            ".#..#.##..".to_string(),
            ".#.##.#..#".to_string(),
            "#.#.#.##.#".to_string(),
            "....#...##".to_string(),
            "...##..##.".to_string(),
            "...#.#####".to_string(),
            ".#.####.#.".to_string(),
            "..#..###.#".to_string(),
            "..##.#..#.".to_string(),
            "".to_string(),
            "Tile 1489:".to_string(),
            "##.#.#....".to_string(),
            "..##...#..".to_string(),
            ".##..##...".to_string(),
            "..#...#...".to_string(),
            "#####...#.".to_string(),
            "#..#.#.#.#".to_string(),
            "...#.#.#..".to_string(),
            "##.#...##.".to_string(),
            "..##.##.##".to_string(),
            "###.##.#..".to_string(),
            "".to_string(),
            "Tile 2473:".to_string(),
            "#....####.".to_string(),
            "#..#.##...".to_string(),
            "#.##..#...".to_string(),
            "######.#.#".to_string(),
            ".#...#.#.#".to_string(),
            ".#########".to_string(),
            ".###.#..#.".to_string(),
            "########.#".to_string(),
            "##...##.#.".to_string(),
            "..###.#.#.".to_string(),
            "".to_string(),
            "Tile 2971:".to_string(),
            "..#.#....#".to_string(),
            "#...###...".to_string(),
            "#.#.###...".to_string(),
            "##.##..#..".to_string(),
            ".#####..##".to_string(),
            ".#..####.#".to_string(),
            "#..#.#..#.".to_string(),
            "..####.###".to_string(),
            "..#.#.###.".to_string(),
            "...#.#.#.#".to_string(),
            "".to_string(),
            "Tile 2729:".to_string(),
            "...#.#.#.#".to_string(),
            "####.#....".to_string(),
            "..#.#.....".to_string(),
            "....#..#.#".to_string(),
            ".##..##.#.".to_string(),
            ".#.####...".to_string(),
            "####.#.#..".to_string(),
            "##.####...".to_string(),
            "##..#.##..".to_string(),
            "#.##...##.".to_string(),
            "".to_string(),
            "Tile 3079:".to_string(),
            "#.#.#####.".to_string(),
            ".#..######".to_string(),
            "..#.......".to_string(),
            "######....".to_string(),
            "####.#..#.".to_string(),
            ".#...#.##.".to_string(),
            "#.#####.##".to_string(),
            "..#.###...".to_string(),
            "..#.......".to_string(),
            "..#.###...".to_string(),
        ];

        let result = get_result_1(&input);
        assert_eq!(result, 20899048083289);
    }

    #[test]
    fn test_get_result_2() {
        let input = vec![
            "Tile 2311:".to_string(),
            "..##.#..#.".to_string(),
            "##..#.....".to_string(),
            "#...##..#.".to_string(),
            "####.#...#".to_string(),
            "##.##.###.".to_string(),
            "##...#.###".to_string(),
            ".#.#.#..##".to_string(),
            "..#....#..".to_string(),
            "###...#.#.".to_string(),
            "..###..###".to_string(),
            "".to_string(),
            "Tile 1951:".to_string(),
            "#.##...##.".to_string(),
            "#.####...#".to_string(),
            ".....#..##".to_string(),
            "#...######".to_string(),
            ".##.#....#".to_string(),
            ".###.#####".to_string(),
            "###.##.##.".to_string(),
            ".###....#.".to_string(),
            "..#.#..#.#".to_string(),
            "#...##.#..".to_string(),
            "".to_string(),
            "Tile 1171:".to_string(),
            "####...##.".to_string(),
            "#..##.#..#".to_string(),
            "##.#..#.#.".to_string(),
            ".###.####.".to_string(),
            "..###.####".to_string(),
            ".##....##.".to_string(),
            ".#...####.".to_string(),
            "#.##.####.".to_string(),
            "####..#...".to_string(),
            ".....##...".to_string(),
            "".to_string(),
            "Tile 1427:".to_string(),
            "###.##.#..".to_string(),
            ".#..#.##..".to_string(),
            ".#.##.#..#".to_string(),
            "#.#.#.##.#".to_string(),
            "....#...##".to_string(),
            "...##..##.".to_string(),
            "...#.#####".to_string(),
            ".#.####.#.".to_string(),
            "..#..###.#".to_string(),
            "..##.#..#.".to_string(),
            "".to_string(),
            "Tile 1489:".to_string(),
            "##.#.#....".to_string(),
            "..##...#..".to_string(),
            ".##..##...".to_string(),
            "..#...#...".to_string(),
            "#####...#.".to_string(),
            "#..#.#.#.#".to_string(),
            "...#.#.#..".to_string(),
            "##.#...##.".to_string(),
            "..##.##.##".to_string(),
            "###.##.#..".to_string(),
            "".to_string(),
            "Tile 2473:".to_string(),
            "#....####.".to_string(),
            "#..#.##...".to_string(),
            "#.##..#...".to_string(),
            "######.#.#".to_string(),
            ".#...#.#.#".to_string(),
            ".#########".to_string(),
            ".###.#..#.".to_string(),
            "########.#".to_string(),
            "##...##.#.".to_string(),
            "..###.#.#.".to_string(),
            "".to_string(),
            "Tile 2971:".to_string(),
            "..#.#....#".to_string(),
            "#...###...".to_string(),
            "#.#.###...".to_string(),
            "##.##..#..".to_string(),
            ".#####..##".to_string(),
            ".#..####.#".to_string(),
            "#..#.#..#.".to_string(),
            "..####.###".to_string(),
            "..#.#.###.".to_string(),
            "...#.#.#.#".to_string(),
            "".to_string(),
            "Tile 2729:".to_string(),
            "...#.#.#.#".to_string(),
            "####.#....".to_string(),
            "..#.#.....".to_string(),
            "....#..#.#".to_string(),
            ".##..##.#.".to_string(),
            ".#.####...".to_string(),
            "####.#.#..".to_string(),
            "##.####...".to_string(),
            "##..#.##..".to_string(),
            "#.##...##.".to_string(),
            "".to_string(),
            "Tile 3079:".to_string(),
            "#.#.#####.".to_string(),
            ".#..######".to_string(),
            "..#.......".to_string(),
            "######....".to_string(),
            "####.#..#.".to_string(),
            ".#...#.##.".to_string(),
            "#.#####.##".to_string(),
            "..#.###...".to_string(),
            "..#.......".to_string(),
            "..#.###...".to_string(),
        ];

        let roughness = get_result_2(&input);
        assert_eq!(roughness, 273);
    }

}