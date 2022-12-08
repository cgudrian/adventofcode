use std::{collections::HashSet, fs};

type TreeHeights = Vec<Vec<u8>>;

fn view_distance_left(tree_heights: &TreeHeights, posx: usize, posy: usize) -> usize {
    if posy == 0 {
        return 0;
    }

    let refheight = tree_heights[posx][posy];
    let mut distance = 0;
    for y in (0..posy).rev() {
        let h = tree_heights[posx][y];
        distance += 1;
        if h >= refheight {
            break;
        }
    }

    distance
}

fn view_distance_right(tree_heights: &TreeHeights, posx: usize, posy: usize) -> usize {
    let refheight = tree_heights[posx][posy];
    let mut distance = 0;
    for y in posy + 1..tree_heights[posx].len() {
        let h = tree_heights[posx][y];
        distance += 1;
        if h >= refheight {
            break;
        }
    }

    distance
}

fn view_distance_up(tree_heights: &TreeHeights, posx: usize, posy: usize) -> usize {
    if posx == 0 {
        return 0;
    }

    let refheight = tree_heights[posx][posy];
    let mut distance = 0;
    for x in (0..posx).rev() {
        let h = tree_heights[x][posy];
        distance += 1;
        if h >= refheight {
            break;
        }
    }

    distance
}

fn view_distance_down(tree_heights: &TreeHeights, posx: usize, posy: usize) -> usize {
    let refheight = tree_heights[posx][posy];
    let mut distance = 0;
    for x in posx + 1..tree_heights.len() {
        let h = tree_heights[x][posy];
        distance += 1;
        if h >= refheight {
            break;
        }
    }

    distance
}

fn scenic_score(tree_heights: &TreeHeights, posx: usize, posy: usize) -> usize {
    let distance_left = view_distance_left(&tree_heights, posx, posy);
    let distance_right = view_distance_right(&tree_heights, posx, posy);
    let distance_up = view_distance_up(&tree_heights, posx, posy);
    let distance_down = view_distance_down(&tree_heights, posx, posy);
    distance_left * distance_right * distance_up * distance_down
}

fn max_scenic_score(tree_heights: &TreeHeights) -> usize {
    let mut max_scenic_score = 0;
    for x in 0..tree_heights.len() {
        for y in 0..tree_heights[x].len() {
            max_scenic_score = max_scenic_score.max(scenic_score(tree_heights, x, y));
        }
    }
    max_scenic_score
}

fn main() {
    let tree_heights: TreeHeights = fs::read("input.txt")
        .unwrap()
        .split(|b| *b == b'\n')
        .filter(|b| b.len() > 0)
        .map(|d| d.iter().map(|b| *b - b'0').collect())
        .collect();

    assert!(tree_heights.len() > 0);

    let sx = tree_heights.len();
    let sy = tree_heights[0].len();

    let edge_count = 2 * (sx + sy) - 4;

    let mut visible_inner_trees: HashSet<(usize, usize)> = HashSet::new();

    for x in 1..sx - 1 {
        let mut maxheight = tree_heights[x][0];
        for y in 1..sy - 1 {
            let height = tree_heights[x][y];
            if height > maxheight {
                maxheight = height;
                visible_inner_trees.insert((x, y));
            }
        }

        let mut maxheight = tree_heights[x][sy - 1];
        for y in (1..sy - 1).rev() {
            let height = tree_heights[x][y];
            if height > maxheight {
                maxheight = height;
                visible_inner_trees.insert((x, y));
            }
        }
    }

    for y in 1..sy - 1 {
        let mut maxheight = tree_heights[0][y];
        for x in 1..sx - 1 {
            let height = tree_heights[x][y];
            if height > maxheight {
                maxheight = height;
                visible_inner_trees.insert((x, y));
            }
        }

        let mut maxheight = tree_heights[sx - 1][y];
        for x in (1..sx - 1).rev() {
            let height = tree_heights[x][y];
            if height > maxheight {
                maxheight = height;
                visible_inner_trees.insert((x, y));
            }
        }
    }

    let total_count = edge_count + visible_inner_trees.len();
    println!("Answer 1: {total_count}");

    let max_scenic_score = max_scenic_score(&tree_heights);
    println!("Answer 2: {max_scenic_score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> TreeHeights {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn test_view_distance_up() {
        assert_eq!(view_distance_up(&data(), 3, 2), 2);
        assert_eq!(view_distance_down(&data(), 3, 2), 1);
        assert_eq!(view_distance_left(&data(), 3, 2), 2);
        assert_eq!(view_distance_right(&data(), 3, 2), 2);
    }

    #[test]
    fn test_scenic_score() {
        assert_eq!(max_scenic_score(&data()), 8);
    }
}
