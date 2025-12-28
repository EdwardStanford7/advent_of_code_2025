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

        let mut max_area_part1 = 0;
        let mut max_area_part2 = 0;

        for tile1 in polygon.vertices.iter() {
            for tile2 in polygon.vertices.iter() {
                let dx = (tile1.x - tile2.x).unsigned_abs() as u64 + 1;
                let dy = (tile1.y - tile2.y).unsigned_abs() as u64 + 1;

                let rectangle_area = dx * dy;
                if rectangle_area > max_area_part1 {
                    max_area_part1 = rectangle_area;
                }

                if rectangle_area > max_area_part2 && polygon.is_valid_rectangle(tile1, tile2) {
                    max_area_part2 = rectangle_area;
                }
            }
        }

        crate::DayResult {
            part_1: max_area_part1,
            part_2: max_area_part2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_surrounding_points(&self) -> Vec<Self> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            self.clone(),
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
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

            polygon_event_points.extend(vertices[i].get_surrounding_points());
        }
        segments.push((vertices[vertices.len() - 1].clone(), vertices[0].clone()));
        polygon_event_points.extend(vertices[vertices.len() - 1].get_surrounding_points());
        Self {
            vertices,
            segments,
            polygon_event_points,
        }
    }

    pub fn is_valid_rectangle(&self, tile1: &Point, tile2: &Point) -> bool {
        self.range_inside(
            &Point {
                x: i32::min(tile1.x, tile2.x),
                y: i32::min(tile1.y, tile2.y),
            },
            &Point {
                x: i32::max(tile1.x, tile2.x),
                y: i32::max(tile1.y, tile2.y),
            },
        )
    }

    fn range_inside(&self, top_left: &Point, bottom_right: &Point) -> bool {
        for point in self
            .polygon_event_points
            .iter()
            .filter(|point| Self::inside_rectangle(point, top_left, bottom_right))
        {
            if !self.inside_polygon(point) {
                return false;
            }
        }

        for point in self
            .get_rectangle_intersections(top_left, bottom_right)
            .iter()
            .filter(|point| Self::inside_rectangle(point, top_left, bottom_right))
        {
            if !self.inside_polygon(point) {
                return false;
            }
        }

        true
    }

    fn inside_rectangle(point: &Point, top_left: &Point, bottom_right: &Point) -> bool {
        top_left.x <= point.x
            && point.x <= bottom_right.x
            && top_left.y <= point.y
            && point.y <= bottom_right.y
    }

    fn get_rectangle_intersections(
        &self,
        top_left: &Point,
        bottom_right: &Point,
    ) -> HashSet<Point> {
        let mut intersection_points = HashSet::new();

        let left_x = top_left.x;
        let right_x = bottom_right.x;
        let top_y = top_left.y;
        let bottom_y = bottom_right.y;

        for (p1, p2) in &self.segments {
            // Vertical polygon segment
            if p1.x == p2.x {
                let x = p1.x;
                let y_min = p1.y.min(p2.y);
                let y_max = p1.y.max(p2.y);

                // Intersect with top edge
                if x > left_x && x < right_x && top_y > y_min && top_y < y_max {
                    intersection_points.extend(Point { x, y: top_y }.get_surrounding_points());
                }

                // Intersect with bottom edge
                if x > left_x && x < right_x && bottom_y > y_min && bottom_y < y_max {
                    intersection_points.extend(Point { x, y: bottom_y }.get_surrounding_points());
                }
            }
            // Horizontal polygon segment
            else {
                let y = p1.y;
                let x_min = p1.x.min(p2.x);
                let x_max = p1.x.max(p2.x);

                // Intersect with left edge
                if y > top_y && y < bottom_y && left_x > x_min && left_x < x_max {
                    intersection_points.extend(Point { x: left_x, y }.get_surrounding_points());
                }

                // Intersect with right edge
                if y > top_y && y < bottom_y && right_x > x_min && right_x < x_max {
                    intersection_points.extend(Point { x: right_x, y }.get_surrounding_points());
                }
            }
        }

        intersection_points
    }

    fn inside_polygon(&self, point: &Point) -> bool {
        let mut num_intersections = 0;

        // Check if the point is inside any segment or if the ray intersects with any segment
        for (p1, p2) in self.segments.iter() {
            if Self::point_inside_segment(point, p1, p2) {
                return true;
            }

            if p1.y != p2.y && // Ignore horizontal segments
             point.x < p1.x && // Raycasting right so point must to left of segment
             // Point is within vertical bounds of segment (only count lower endpoints)
             ((p1.y <= point.y && point.y < p2.y)||(p2.y <= point.y && point.y < p1.y))
            {
                num_intersections += 1;
            }
        }

        num_intersections % 2 == 1
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
