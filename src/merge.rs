use Difference;

// merges the changes from two strings, given a common substring
pub fn merge (orig: &str, edit: &str, common: &str, split: &str) -> Vec<Difference> {
    let mut ret = Vec::new();

    let mut a = orig.split(split);
    let mut b = edit.split(split);

    let mut same = String::new();
    for c in common.split(split) {
        let mut add = String::new();
        let mut rem = String::new();

        let mut x = a.next();
        while x != None && Some(c) != x {
            if rem.len() > 0 {
                rem.push_str(split);
            }
            rem.push_str(x.unwrap());
            x = a.next();
        }

        let mut y = b.next();
        while y != None && Some(c) != y {
            if add.len() > 0 {
                add.push_str(split);
            }
            add.push_str(y.unwrap());
            y = b.next();
        }

        if add.len() > 0 || rem.len() > 0 {
            ret.push(Difference::Same(same.clone()));
            same.clear();
        }

        if rem.len() > 0 {
            ret.push(Difference::Rem(rem.clone()));
        }

        if add.len() > 0 {
            ret.push(Difference::Add(add.clone()));
        }

        if same.len() > 0 {
            same.push_str(split);
        }
        same.push_str(c);
    }
    if same.len() > 0 {
        ret.push(Difference::Same(same.clone()));
    }

    // TODO avoid duplication

    let mut rem = String::new();

    for x in a {
        if rem.len() > 0 {
            rem.push_str(split);
        }
        rem.push_str(x);
    }
    if rem.len() > 0 {
        ret.push(Difference::Rem(rem.clone()));
    }

    let mut add = String::new();
    for y in b {
        if add.len() > 0 {
            add.push_str(split);
        }
        add.push_str(y);
    }
    if add.len() > 0 {
        ret.push(Difference::Add(add.clone()));
    }

    ret
}


#[test]
fn test_merge() {
    assert_eq!(merge("testa", "tost", "tst", ""), vec![
               Difference::Same("t".to_string()),
               Difference::Rem("e".to_string()),
               Difference::Add("o".to_string()),
               Difference::Same("st".to_string()),
               Difference::Rem("a".to_string()),
    ]);
}
