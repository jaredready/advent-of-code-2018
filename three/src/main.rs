#[derive(Eq, PartialEq, Debug)]
struct Rectangle {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    id: String,
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let claims: Vec<Rectangle> = INPUT.lines().map(|claim| build_rectangle(claim)).collect();
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for claim in &claims {
        for x_offset in 0..claim.width {
            for y_offset in 0..claim.height {
                grid[claim.x + x_offset][claim.y + y_offset] += 1;
            }
        }
    }

    let mut non_overlapped_claim = "NONE";
    for claim_a in &claims {
        let mut overlapped = false;
        for claim_b in &claims {
            if claim_a == claim_b {
                continue;
            }
            if intersects(claim_a, claim_b) {
                overlapped = true;
                println!("{:?} intersects {:?}", claim_a, claim_b);
                break;
            }
        }

        if !overlapped {
            non_overlapped_claim = &claim_a.id;
        }
    }

    println!(
        "P1: {:?}",
        grid.iter()
            .flatten()
            .filter(|claim_count| claim_count >= &&2)
            .collect::<Vec<&usize>>()
            .len()
    );

    println!("{:?}", non_overlapped_claim);
}

fn intersects(a: &Rectangle, b: &Rectangle) -> bool {
    if a.x + a.width < b.x {
        return false;
    }
    if a.x > b.x + b.width {
        return false;
    }
    if a.y + a.height < b.y {
        return false;
    }
    if a.y > b.y + b.height {
        return false;
    }

    return true;
}

fn build_rectangle(claim: &str) -> Rectangle {
    let coordinates = claim
        .split(' ')
        .find(|token| token.contains(":"))
        .unwrap()
        .split(":")
        .nth(0)
        .unwrap();
    let size = claim.split(' ').find(|token| token.contains("x")).unwrap();

    let x = coordinates.split(",").nth(0).unwrap();
    let y = coordinates.split(",").nth(1).unwrap();

    let width = size.split("x").nth(0).unwrap();
    let height = size.split("x").nth(1).unwrap();

    let id = claim.split(' ').nth(0).unwrap();

    return Rectangle {
        width: width.parse::<usize>().unwrap(),
        height: height.parse::<usize>().unwrap(),
        x: x.parse::<usize>().unwrap(),
        y: y.parse::<usize>().unwrap(),
        id: id.to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects_true() {
        let a = Rectangle {
            x: 1,
            y: 3,
            width: 4,
            height: 5,
        };
        let b = Rectangle {
            x: 2,
            y: 2,
            width: 4,
            height: 5,
        };

        assert_eq!(intersects(a, b), true);
    }

    #[test]
    fn test_intersects_false() {
        let a = Rectangle {
            x: 1,
            y: 3,
            width: 4,
            height: 5,
        };
        let b = Rectangle {
            x: 200,
            y: 2,
            width: 4,
            height: 5,
        };

        assert_eq!(intersects(a, b), false);
    }

    #[test]
    fn test_build_rectangle() {
        let input = "#1 @ 1,3: 4x5";
        let rectangle = build_rectangle(input);

        let expected = Rectangle {
            x: 1,
            y: 3,
            width: 4,
            height: 5,
        };

        assert_eq!(rectangle, expected);
    }
}
