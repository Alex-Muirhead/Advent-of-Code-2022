fn main() {
    let lines = include_str!("../../data/day_08.txt").lines();
    let width = 99; // From Python counting: 99

    let mut forrest: Vec<i32> = Vec::new();

    // Parse lines into forrest
    for line in lines {
        let numbers: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        forrest.extend(numbers);
    }

    let num_trees = forrest.len();
    let mut visible_trees = 0;
    let mut best_score = 0;

    // Brute force approach first

    for position in 0..num_trees {
        let row = position.div_euclid(width);
        let col = position.rem_euclid(width);

        let tree = forrest[position];
        let tree_row = &forrest[row*width..(row+1)*width];
        let tree_col: Vec<&i32> = forrest[col..].iter().step_by(width).collect();

        let (left, right) = tree_row.split_at(col);
        let (top, bottom) = tree_col.split_at(row);

        if left.iter().all(|&l| tree > l)
            || right[1..].iter().all(|&r| tree > r)
            || top.iter().all(|&&t| tree > t)
            || bottom[1..].iter().all(|&&b| tree > b) {
            // println!("{} Can be seen!", tree);
            visible_trees += 1
        }

        let left_score   = left.iter().rev().position(|l| l >= &tree).map_or(left.len(), |p| p+1);
        let top_score    = top.iter().rev().position(|&t| t >= &tree).map_or(top.len(), |p| p+1);
        let right_score  = right[1..].iter().position(|r| r >= &tree).map_or(right.len()-1, |p| p+1);
        let bottom_score = bottom[1..].iter().position(|&b| b >= &tree).map_or(bottom.len()-1, |p| p+1);

        let score = left_score * right_score * top_score * bottom_score;
        if score > best_score {
            best_score = score;
        }
    }

    println!("There are {} visible trees", visible_trees);
    println!("The best scored tree is {}", best_score);
}
