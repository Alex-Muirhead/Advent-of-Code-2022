use std::collections::HashSet;

#[derive(Debug, Default, Eq, Hash, PartialEq, Clone)]
struct State {
    x: i32,
    y: i32,
}

impl State {
    fn move_head(&mut self, dir: &str) {
        match dir {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!("Unknown direction {dir}!")
        }
    }

    fn drag(&self, other: &mut State) -> bool {
        let offset_x = self.x - other.x;
        let offset_y = self.y - other.y;

        if offset_x.abs() > 1 || offset_y.abs() > 1 {
            other.x += offset_x.signum();
            other.y += offset_y.signum();
            return true;
        }

        false
    }
}

fn main() {
    let knot_length = 10;

    let lines = include_str!("../../data/day_09.txt").lines();
    let mut knots = vec![State::default(); knot_length];

    let mut visited_states: HashSet<State> = HashSet::new();
    visited_states.insert(knots.last().unwrap().clone());

    for line in lines {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist: i32 = dist.parse().unwrap();

        for _ in 0..dist {

            let lead = knots.first_mut().unwrap();
            lead.move_head(dir);

            for i in 0..(knot_length-1) {
                let lead = knots.get(i).unwrap().clone();
                let next = knots.get_mut(i+1).unwrap();

                // If dragging made no changes, no others shall occur
                if !lead.drag(next) {
                    continue;
                }
            }

            visited_states.insert(knots.last().unwrap().clone());
        }
    }

    println!("A total of {} states visited", visited_states.len());
}