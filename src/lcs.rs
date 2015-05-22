// finds the longest common subsequences
// outputs the edit distance and a string containing
// all chars both inputs have in common
pub fn lcs(orig: &str, edit: &str, split: &str) -> (i32, String) {

    let a : Vec<&str> = orig.split(split).collect();
    let b : Vec<&str> = edit.split(split).collect();

    let N = a.len() as i32;
    let M = b.len() as i32;

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
                snake.push_str(a[x as usize]);
                snake.push_str(split);
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
    assert_eq!(lcs("test", "tost", ""), (2, "tst".to_string()));
    assert_eq!(lcs("The quick brown fox jumps over the lazy dog", "The quick brown dog leaps over the lazy cat", ""), (16, "The quick brown o ps over the lazy ".to_string()));
    assert_eq!(lcs("The quick brown fox jumps over the lazy dog", "The quick brown dog leaps over the lazy cat", " "), (6, "The quick brown over the lazy ".to_string()));
}
