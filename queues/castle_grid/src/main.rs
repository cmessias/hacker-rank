use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufRead;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq)]
enum Tile {
    Space,
    Blocked,
}

fn fewest_moves(grid: &Vec<Vec<Tile>>, start: Point, goal: Point) -> Option<u32> {
    let mut to_visit_neighbors = VecDeque::<Point>::new();
    let mut visited = HashSet::<Point>::new();
    let mut moves = HashMap::<Point, u32>::new();

    to_visit_neighbors.push_back(start);
    visited.insert(start);
    moves.insert(start, 0);

    while let Some(node) = to_visit_neighbors.pop_front() {
        for child in get_possible_moves(node, &grid, &mut moves).iter() {
            if !visited.contains(&child) {
                if *child == goal {
                    return Some(*moves.get(&node).unwrap() + 1);
                }

                visited.insert(*child);
                to_visit_neighbors.push_back(*child);
            }
        }
    }

    return None;
}

fn get_possible_moves(
    position: Point,
    grid: &Vec<Vec<Tile>>,
    moves: &mut HashMap<Point, u32>,
) -> Vec<Point> {
    let moves_north: Vec<Point> = (0..=position.x)
        .rev()
        .take_while(|x| grid[*x][position.y] == Tile::Space)
        .map(|x| Point::new(x, position.y))
        .inspect(|next_position| update_move_cost(position, *next_position, moves))
        .collect();

    let moves_south: Vec<Point> = (position.x..grid.len())
        .take_while(|x| grid[*x][position.y] == Tile::Space)
        .map(|x| Point::new(x, position.y))
        .inspect(|next_position| update_move_cost(position, *next_position, moves))
        .collect();

    let moves_west: Vec<Point> = (0..=position.y)
        .rev()
        .take_while(|y| grid[position.x][*y] == Tile::Space)
        .map(|y| Point::new(position.x, y))
        .inspect(|next_position| update_move_cost(position, *next_position, moves))
        .collect();

    let moves_east: Vec<Point> = (position.y..grid.len())
        .take_while(|y| grid[position.x][*y] == Tile::Space)
        .map(|y| Point::new(position.x, y))
        .inspect(|next_position| update_move_cost(position, *next_position, moves))
        .collect();

    return [moves_north, moves_south, moves_west, moves_east].concat();
}

fn update_move_cost(from: Point, to: Point, moves: &mut HashMap<Point, u32>) {
    if !moves.contains_key(&to) {
        moves.insert(to, *moves.get(&from).unwrap() + 1);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let grid: Vec<String> = io::stdin()
        .lock()
        .lines()
        .take(n)
        .map(|s| s.unwrap().trim().to_string())
        .collect();

    let mut positions = String::with_capacity(7);
    io::stdin().read_line(&mut positions).unwrap();
    let positions: Vec<usize> = positions
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let (start_x, start_y) = (positions[0], positions[1]);
    let (goal_x, goal_y) = (positions[2], positions[3]);

    let grid = build_grid_map(&grid);

    println!(
        "{}",
        fewest_moves(
            &grid,
            Point::new(start_x, start_y),
            Point::new(goal_x, goal_y)
        )
        .unwrap()
    )
}

fn build_grid_map(grid: &[String]) -> Vec<Vec<Tile>> {
    return grid.iter().map(|row| build_row(row)).collect();
}

fn build_row(row: &str) -> Vec<Tile> {
    return row
        .chars()
        .map(|c| match c {
            '.' => Tile::Space,
            'X' => Tile::Blocked,
            _ => panic!(),
        })
        .collect();
}
