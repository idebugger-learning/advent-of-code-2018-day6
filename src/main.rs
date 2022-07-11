use std::collections::{HashMap, HashSet};

type Point = (isize, isize);

fn main() {
    // let input = include_str!("./data/input_example.txt");
    let input = include_str!("./data/input.txt");
    let points = parse_input(input);
    println!("Points ({}): {:?}", points.len(), points);

    let infinite_points = find_infinite_points(&points);
    println!(
        "Infinite points ({}): {:?}",
        infinite_points.len(),
        infinite_points
    );

    let finite_points: HashSet<Point> = points
        .difference(&infinite_points)
        .map(|&point| point)
        .collect();
    println!(
        "Finite points ({}): {:?}",
        finite_points.len(),
        finite_points
    );

    let largest_area_size = find_largest_area(&points, &finite_points);
    println!("Largest area size: {}", largest_area_size);
}

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .split("\n")
        .map(|line| {
            line.split(", ")
                .map(|raw_num| {
                    raw_num
                        .parse::<isize>()
                        .expect("Failed to parse number for point")
                })
                .collect::<Vec<_>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect()
}

fn find_min_max(points: &HashSet<Point>) -> (isize, isize, isize, isize) {
    let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
    let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);

    for (x, y) in points {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    (min_x, max_x, min_y, max_y)
}

fn find_infinite_points(points: &HashSet<Point>) -> HashSet<Point> {
    let (min_x, max_x, min_y, max_y) = find_min_max(points);

    let diff_x = max_x.abs_diff(min_x);
    let from_x = min_x - diff_x as isize;
    let to_x = max_x + diff_x as isize;

    let diff_y = max_y.abs_diff(min_y);
    let from_y = min_y - diff_y as isize;
    let to_y = max_y + diff_y as isize;

    let mut infinite_points = HashSet::new();

    for x in min_x..=max_x {
        let closest_point = find_closest_point(points, &(x, from_y));
        infinite_points.insert(closest_point);
        let closest_point = find_closest_point(points, &(x, to_y));
        infinite_points.insert(closest_point);
    }

    for y in min_y..=max_y {
        let closest_point = find_closest_point(points, &(y, from_x));
        infinite_points.insert(closest_point);
        let closest_point = find_closest_point(points, &(y, to_x));
        infinite_points.insert(closest_point);
    }

    infinite_points
}

fn find_closest_point(points: &HashSet<Point>, target: &Point) -> Point {
    *points
        .into_iter()
        .min_by_key(|point| calc_manhattan_distance(point, target))
        .expect("Can't find closest point")
}

fn calc_manhattan_distance(from: &Point, to: &Point) -> isize {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as isize
}

fn find_largest_area(points: &HashSet<Point>, finite_points: &HashSet<Point>) -> usize {
    let (min_x, max_x, min_y, max_y) = find_min_max(points);

    let mut point_counters: HashMap<Point, usize> = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let closest_point = find_closest_point(points, &(x, y));
            if finite_points.contains(&closest_point) {
                *point_counters.entry(closest_point).or_insert(0) += 1;
            }
        }
    }

    point_counters
        .into_values()
        .max()
        .expect("Failed to find max")
}
