use std::{collections::HashSet, hash::Hash};

pub struct Day9;

impl crate::Day for Day9 {
    fn run(input: String) -> crate::DayResult {
        let red_tiles = input
            .lines()
            .map(|line| {
                let (x, y) = line.trim().split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect::<Vec<Point>>();

        let polygon = Polygon::new(red_tiles);

        let mut max_size_part1 = 0;
        let mut max_size_part2 = 0;
        let mut max_rectangle = (&Point { x: 0, y: 0 }, &Point { x: 0, y: 0 });

        for tile1 in polygon.vertices.iter() {
            for tile2 in polygon.vertices.iter() {
                let dx = (tile1.x - tile2.x).unsigned_abs() as u64 + 1;
                let dy = (tile1.y - tile2.y).unsigned_abs() as u64 + 1;

                let rectangle_size = dx * dy;
                if rectangle_size > max_size_part1 {
                    max_size_part1 = rectangle_size;
                }

                if polygon.is_valid_rectangle(tile1, tile2) && rectangle_size > max_size_part2 {
                    max_size_part2 = rectangle_size;
                    max_rectangle = (tile1, tile2);
                }
            }
        }

        println!(
            "Max rectangle: {:?} to {:?}",
            max_rectangle.0, max_rectangle.1
        );
        println!("\n\n{}\n\n", Grid::new(&polygon.vertices));

        crate::DayResult {
            part_1: max_size_part1,
            part_2: max_size_part2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Polygon {
    vertices: Vec<Point>,
    segments: Vec<(Point, Point)>,
    polygon_event_points: HashSet<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        let mut segments = Vec::new();
        let mut polygon_event_points = HashSet::new();
        for i in 0..vertices.len() - 1 {
            segments.push((vertices[i].clone(), vertices[i + 1].clone()));

            polygon_event_points.extend(Self::get_surrounding_points(&vertices[i]));
        }
        segments.push((vertices[vertices.len() - 1].clone(), vertices[0].clone()));
        polygon_event_points.extend(Self::get_surrounding_points(&vertices[vertices.len() - 1]));
        Self {
            vertices,
            segments,
            polygon_event_points,
        }
    }

    fn get_surrounding_points(p1: &Point) -> Vec<Point> {
        vec![
            Point {
                x: p1.x - 1,
                y: p1.y - 1,
            },
            Point {
                x: p1.x - 1,
                y: p1.y,
            },
            Point {
                x: p1.x - 1,
                y: p1.y + 1,
            },
            Point {
                x: p1.x,
                y: p1.y - 1,
            },
            Point { x: p1.x, y: p1.y },
            Point {
                x: p1.x,
                y: p1.y + 1,
            },
            Point {
                x: p1.x + 1,
                y: p1.y - 1,
            },
            Point {
                x: p1.x + 1,
                y: p1.y,
            },
            Point {
                x: p1.x + 1,
                y: p1.y + 1,
            },
        ]
    }

    pub fn is_valid_rectangle(&self, tile1: &Point, tile2: &Point) -> bool {
        let range = if tile1.x < tile2.x {
            if tile1.y < tile2.y {
                (tile1.x, tile2.x, tile1.y, tile2.y)
            } else {
                (tile1.x, tile2.x, tile2.y, tile1.y)
            }
        } else if tile1.y < tile2.y {
            (tile2.x, tile1.x, tile1.y, tile2.y)
        } else {
            (tile2.x, tile1.x, tile2.y, tile1.y)
        };

        self.range_inside(range.0, range.1, range.2, range.3)
    }

    fn range_inside(&self, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
        let mut new_event_points = HashSet::new();

        for point in self
            .polygon_event_points
            .iter()
            // Only count event points that are actually in the rectangle we are checking
            .filter(|point| x1 <= point.x && point.x <= x2 && y1 <= point.y && point.y <= y2)
        {
            let (inside, new_points) = self.inside_polygon(Point {
                x: point.x,
                y: point.y,
            });
            new_event_points.extend(new_points.into_iter());
            if !inside {
                return false;
            }
        }

        // for point in new_event_points
        //     .iter()
        //     .filter(|point| x1 <= point.x && point.x <= x2 && y1 <= point.y && point.y <= y2)
        // {
        //     if !self
        //         .inside_polygon(Point {
        //             x: point.x,
        //             y: point.y,
        //         })
        //         .0
        //     {
        //         return false;
        //     }
        // }

        true
    }

    fn inside_polygon(&self, point: Point) -> (bool, HashSet<Point>) {
        let mut num_intersections = 0;
        let mut new_event_points = HashSet::new();
        let mut inside_segment = false;

        // Check if the point is inside any segment or if the ray intersects with any segment
        for (p1, p2) in self.segments.iter() {
            if Self::point_inside_segment(&point, p1, p2) {
                inside_segment = true;
            }

            if p1.y != p2.y && // Ignore horizontal segments
             point.x < p1.x && // Raycasting right so point must to left of segment
             // Point is within vertical bounds of segment (only count lower endpoints)
             ((p1.y <= point.y && point.y < p2.y)||(p2.y <= point.y && point.y < p1.y))
            {
                num_intersections += 1;
                new_event_points.extend(Self::get_surrounding_points(&Point {
                    x: point.x,
                    y: p1.y,
                }));
            }
        }

        (
            inside_segment || num_intersections % 2 == 1,
            new_event_points,
        )
    }

    fn point_inside_segment(point: &Point, segment_start: &Point, segment_end: &Point) -> bool {
        (segment_start.x == segment_end.x
            && segment_end.x == point.x
            && ((segment_start.y <= point.y && point.y <= segment_end.y)
                || (segment_end.y <= point.y && point.y <= segment_start.y)))
            || (segment_start.y == segment_end.y
                && segment_end.y == point.y
                && ((segment_start.x <= point.x && point.x <= segment_end.x)
                    || (segment_end.x <= point.x && point.x <= segment_start.x)))
    }
}

// fn get_point_after(
//     point: &Point,
//     segment_start: &Point,
//     segment_end: &Point,
//     direction: Direction,
// ) -> Point {
//     match direction {
//         Direction::Up => Point {
//             x: point.x,
//             y: i32::min(segment_start.y, segment_end.y) - 1,
//         },
//         Direction::Down => Point {
//             x: point.x,
//             y: i32::max(segment_start.y, segment_end.y) + 1,
//         },
//         Direction::Left => Point {
//             x: i32::min(segment_start.x, segment_end.x) - 1,
//             y: point.y,
//         },
//         Direction::Right => Point {
//             x: i32::max(segment_start.x, segment_end.x) + 1,
//             y: point.y,
//         },
//     }
// }

// fn ray_intersects(
//     origin: &Point,
//     segment_start: &Point,
//     segment_end: &Point,
//     direction: Direction,
// ) -> bool {
//     match direction {
//         Direction::Up => {
//             segment_start.x != segment_end.x && // Ignore vertical segments
//             origin.y < segment_start.y && // Origin is below horizontal segment
//             ((segment_start.x <= origin.x && origin.x < segment_end.x) // Origin is within bounds of segment
//                 || (segment_end.x <= origin.x && origin.x < segment_start.x)) // Count lower endpoints but not upper endpoints
//         }
//         Direction::Down => {
//             segment_start.x != segment_end.x && // Ignore vertical segments
//             origin.y > segment_start.y && // Origin is above horizontal segment
//             ((segment_start.x <= origin.x && origin.x < segment_end.x) // Origin is within bounds of segment
//                 || (segment_end.x <= origin.x && origin.x < segment_start.x)) // Count lower endpoints but not upper endpoints
//         }
//         Direction::Left => {
//             segment_start.y != segment_end.y && // Ignore horizontal segments
//             origin.x > segment_start.x && // Origin is to the right of vertical segment
//             ((segment_start.y <= origin.y && origin.y < segment_end.y) // Origin is within bounds of segment
//                 || (segment_end.y <= origin.y && origin.y < segment_start.y)) // Count lower endpoints but not upper endpoints
//         }
//         Direction::Right => {
//             segment_start.y != segment_end.y && // Ignore horizontal segments
//             origin.x > segment_start.x && // Origin is to the left of vertical segment
//             ((segment_start.y <= origin.y && origin.y < segment_end.y) // Origin is within bounds of segment
//                 || (segment_end.y <= origin.y && origin.y < segment_start.y)) // Count lower endpoints but not upper endpoints
//         }
//     }
// }

struct Grid {
    grid: Vec<Vec<TileColor>>,
}

impl Grid {
    pub fn new(red_tiles: &[Point]) -> Self {
        // Make grid
        let mut grid =
            vec![
                vec![TileColor::None; red_tiles.iter().map(|tile| tile.x).max().unwrap() as usize];
                red_tiles.iter().map(|tile| tile.y).max().unwrap() as usize
            ];

        // Fill in outline
        for i in 0..red_tiles.len() - 1 {
            let tile1 = &red_tiles[i];
            grid[tile1.y as usize - 1][tile1.x as usize - 1] = TileColor::Red;
            let tile2 = &red_tiles[i + 1];

            if tile1.x == tile2.x {
                for y in (tile1.y.min(tile2.y) + 1)..tile1.y.max(tile2.y) {
                    grid[y as usize - 1][tile1.x as usize - 1] = TileColor::Green;
                }
            } else {
                for x in (tile1.x.min(tile2.x) + 1)..tile1.x.max(tile2.x) {
                    grid[tile1.y as usize - 1][x as usize - 1] = TileColor::Green;
                }
            }
        }

        // Make last connection between first and last tiles
        let tile1 = &red_tiles[red_tiles.len() - 1];
        grid[tile1.y as usize - 1][tile1.x as usize - 1] = TileColor::Red;
        let tile2 = &red_tiles[0];

        if tile1.x == tile2.x {
            for y in (tile1.y.min(tile2.y) + 1)..tile1.y.max(tile2.y) {
                grid[y as usize - 1][tile1.x as usize - 1] = TileColor::Green;
            }
        } else {
            for x in (tile1.x.min(tile2.x) + 1)..tile1.x.max(tile2.x) {
                grid[tile1.y as usize - 1][x as usize - 1] = TileColor::Green;
            }
        }

        Grid { grid }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                let c = match tile {
                    TileColor::Red => "# ",
                    TileColor::Green => "X ",
                    TileColor::None => ". ",
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TileColor {
    Red,
    Green,
    None,
}
