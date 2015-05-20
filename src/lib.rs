#![feature(step_by)]

pub fn lcs(orig: &str, edit: &str) -> i32 {
    let N = orig.len() as i32;
    let M = edit.len() as i32;

    let a : Vec<char> = orig.chars().collect();
    let b : Vec<char> = edit.chars().collect();

    let MAX = N + M;

    let mut v : Vec<i32> = (-MAX..MAX).collect();

    //let common = Vec::new();

    v[1] = 0;

    for D in 0..MAX {
        let mut max = 0;
        let mut max_snake : Box<Vec<i32>>;

        for k in (-D..D+1).step_by(2) {
            let snake = Vec::new();
            max_snake = Box::new(snake);

            let mut x;

            let index = (MAX + k) as usize;
            if k == -D || k != D && v[index - 1] < v[index + 1]{
                x = v[index + 1];
            }else{
                x = v[index - 1] + 1;
            }

            let mut y = x - k;

            while x < N && y < M && a[x as usize] == b[y as usize] {
                x += 1;
                y += 1;
            }

            v[index] = x;

            if x >= N && y >= M {
                return D;
            }
        }
    }

    unreachable!()
}

#[test]
fn test_lcs() {
    assert_eq!(lcs("test", "tost"), 2);
    assert_eq!(lcs("The quick brown fox jumps over the lazy dog", "The quick brown dog leaps over the lazy cat"), 16);
}
