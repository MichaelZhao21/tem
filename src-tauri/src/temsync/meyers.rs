use super::{
    edit::{Edit, EditType},
    token::Token,
};

/// Use the Meyer's diffing algorithm to find all edits required to get from
/// the prev to curr file state. See the algorithm here:
/// https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/
pub fn get_diff(prev: &Vec<Token>, curr: &Vec<Token>) -> Result<Vec<Edit>, String> {
    // Define variables and dp array
    let n = prev.len();
    let m = curr.len();
    let max = n + m;
    let max_size = max * 2 + 1;
    let mut dp = vec![usize::MAX; max_size];
    let mut trace = Vec::<Vec<usize>>::new();

    // Set first dp value to get (0, 0) on first calculation
    dp[max] = 0;

    // Fill dp array, going left to right on d and top to bottom on k
    for d in 0..max {
        // Push previous state to array
        trace.push(dp.clone());

        // Go from -d to d, but offset because usize indexes
        for ki in (0..(2 * d + 1)).step_by(2) {
            // We know k = ki - d bc ki is offset by d
            // Also idx = k + max - 1, subbing for k we get
            let idx = ki + max - d - 1;

            // x = down (same) or right (prev + 1)
            let mut x = if ki == 0 || (ki != (2 * d) && dp[idx - 1] < dp[idx + 1]) {
                dp[idx + 1]
            } else {
                dp[idx - 1] + 1
            };

            // Because idx = k + max, y = x - (idx - max)
            // This is the same as y = x - k
            let mut y = x + max - 1 - idx;

            // Diagonal steps
            while x < n && y < m && prev[x].eq_value(&curr[y]) {
                (x, y) = (x + 1, y + 1);
            }

            // Store x in current k
            dp[idx] = x;

            // We are done if we've reached bottom right (n, m)
            if x >= n && y >= m {
                let path = backtrack(&trace, n, m);
                return Ok(gen_edits(&path, &prev, &curr));
            }
        }
    }

    Err("unable to reach the final string state".into())
}

#[derive(Debug)]
struct Move {
    pub prev_x: usize,
    pub prev_y: usize,
    pub x: usize,
    pub y: usize,
}

impl Move {
    pub fn new(prev_x: usize, prev_y: usize, x: usize, y: usize) -> Self {
        Move {
            prev_x,
            prev_y,
            x,
            y,
        }
    }
}

fn backtrack(trace: &Vec<Vec<usize>>, n: usize, m: usize) -> Vec<Move> {
    // Get max and starting (x, y)
    let max = n + m;
    let (mut x, mut y) = (n, m);

    // Create output
    let mut path = Vec::<Move>::with_capacity(trace.len());

    // Iterate through trace backwards
    for (d, t) in trace.iter().enumerate().rev() {
        // Same as above logic, k = x - y and idx = k + max - 1
        // So to keep usize, we do idx = x - y + max - 1
        // We can apply the same logic backwards to get ki (ki = idx + d - (max - 1)) => (ki = x - y + d)
        let idx = x + max - y - 1;
        let ki = x + d - y;

        // Using same logic as above, determine if we take a rightwards or downwards step
        let prev_idx = if ki == 0 || (ki != (2 * d) && t[idx - 1] < t[idx + 1]) {
            idx + 1
        } else {
            idx - 1
        };

        // Get previous x and y
        let prev_x = t[prev_idx];
        let prev_y = prev_x + max - 1 - prev_idx;

        // Step backwards diagonally
        while x > prev_x && y > prev_y {
            path.push(Move::new(x - 1, y - 1, x, y));
            (x, y) = (x - 1, y - 1);
        }

        // Add a move from prev to current
        path.push(Move::new(prev_x, prev_y, x, y));

        (x, y) = (prev_x, prev_y);

        // If we reach (0, 0) break
        // Otherwise, if we reach the end (d = 1), then go diagonally down to (0, 0)
        if x == 0 && y == 0 {
            break;
        } else if d == 1 {
            // This should never happen
            if x != y {
                panic!("invalid state for backtracking -- x and y should match");
            }

            // Diagonal steps back to 0,0
            while x > 0 {
                path.push(Move::new(x - 1, y - 1, x, y));
                (x, y) = (x - 1, y - 1);
            }
            break;
        }
    }

    // Return reversed path
    path
}

fn gen_edits(path: &Vec<Move>, prev: &Vec<Token>, curr: &Vec<Token>) -> Vec<Edit> {
    // Create output
    let mut out = Vec::<Edit>::new();

    // Reverse loop through backtrack
    for m in path.iter().rev() {
        // Figure out if it was a deletion, addition, or unchanged
        if m.x == m.prev_x {
            push_or_combine(
                &mut out,
                EditType::INSERT,
                &curr[m.prev_y],
                m.prev_x,
                m.prev_y,
            );
        } else if m.y == m.prev_y {
            push_or_combine(
                &mut out,
                EditType::DELETE,
                &prev[m.prev_x],
                m.prev_x,
                m.prev_y,
            );
        } else {
            push_or_combine(
                &mut out,
                EditType::SAME,
                &prev[m.prev_x],
                m.prev_x,
                m.prev_y,
            )
        }
    }

    out
}

fn push_or_combine(out: &mut Vec<Edit>, edit_type: EditType, token: &Token, x: usize, y: usize) {
    if !out.is_empty() && out.last().unwrap().edit_type == edit_type {
        out.last_mut().unwrap().append_token(token.clone());
    } else {
        out.push(Edit::new(x, y, edit_type, token.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff() {
        // Create 2 sets of tokens
        let a = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 2, "B".into()),
            Token::new_with_values(2, 3, "C".into()),
            Token::new_with_values(3, 4, "D".into()),
        ];
        let b = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 2, "G".into()),
            Token::new_with_values(2, 3, "Y".into()),
            Token::new_with_values(3, 4, "C".into()),
        ];

        let diff = get_diff(&a, &b).expect("meyers get_diff failed");
        println!("{:?}", diff);

        assert_eq!(diff.len(), 5, "diff should be length 5");
    }

    #[test]
    fn test_get_diff_big() {
        // Create 2 sets of tokens
        let a = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 1, " ".into()),
            Token::new_with_values(2, 3, "car".into()),
            Token::new_with_values(5, 1, " ".into()),
            Token::new_with_values(6, 3, "ate".into()),
            Token::new_with_values(9, 1, " ".into()),
            Token::new_with_values(10, 2, "my".into()),
            Token::new_with_values(12, 1, " ".into()),
            Token::new_with_values(13, 3, "dog".into()),
            Token::new_with_values(16, 1, ".".into()),
        ];
        let b = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 1, " ".into()),
            Token::new_with_values(2, 3, "red".into()),
            Token::new_with_values(5, 1, " ".into()),
            Token::new_with_values(6, 3, "car".into()),
            Token::new_with_values(9, 1, " ".into()),
            Token::new_with_values(10, 5, "eaten".into()),
            Token::new_with_values(15, 1, " ".into()),
            Token::new_with_values(16, 2, "my".into()),
        ];

        let diff = get_diff(&a, &b).expect("meyers get_diff failed");
        println!("{:?}", diff);

        assert_eq!(diff.len(), 7, "diff should be length 7");
    }
}
