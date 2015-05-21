#![feature(step_by)]

#[derive(PartialEq, Debug)]
pub enum Difference {
    Same(String),
    Add(String),
    Rem(String)
}

fn lcs(orig: &str, edit: &str) -> (i32, String) {
    let N = orig.len() as i32;
    let M = edit.len() as i32;

    let a : Vec<char> = orig.chars().collect();
    let b : Vec<char> = edit.chars().collect();

    let MAX = N + M;

    let mut v : Vec<i32> = (-MAX..MAX).collect();

    let mut common = String::new();

    v[1] = 0;

    for D in 0..MAX {
        let mut max = 0;
        let mut max_snake : Box<String> = Box::new("".to_string());

        for k in (-D..D+1).step_by(2) {
            let mut snake = String::new();

            let mut x;

            let index = (MAX + k - 1) as usize;
            if k == -D || k != D && v[index - 1] < v[index + 1]{
                x = v[index + 1];
            }else{
                x = v[index - 1] + 1;
            }

            let mut y = x - k;

            while x < N && y < M && a[x as usize] == b[y as usize] {
                snake.push(a[x as usize]);
                x += 1;
                y += 1;
            }

            v[index] = x;

            if x > max {
                max = x;
                max_snake = Box::new(snake);
            }

            if x >= N && y >= M {
                if max_snake.len() > 0 {
                    common.push_str(&max_snake);
                }
                return (D, common);
            }
        }

        if max_snake.len() > 0 {
            common.push_str(&max_snake);
        }

    }

    unreachable!()
}

#[test]
fn test_lcs() {
    assert_eq!(lcs("test", "tost"), (2, "tst".to_string()));
    assert_eq!(lcs("The quick brown fox jumps over the lazy dog", "The quick brown dog leaps over the lazy cat"), (16, "The quick brown o ps over the lazy ".to_string()));
}
