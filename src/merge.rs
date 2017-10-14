use Difference;

// merges the changes from two strings, given a common substring
pub fn merge(orig: &str, edit: &str, common: &str, split: &str) -> Vec<Difference> {
    if common == "" {
        return merge_without_common(orig, edit, split)
    };

    let mut ret = Vec::new();

    let mut l = orig.split(split).peekable();
    let mut r = edit.split(split).peekable();
    let mut c = common.split(split).peekable();

    // Turn empty strings into [], not [""]
    if orig == "" {
        l.next();
    }
    if edit == "" {
        r.next();
    }

    while l.peek().is_some() || r.peek().is_some() {
        let mut same = Vec::new();
        while l.peek().is_some() && l.peek() == c.peek() && r.peek() == c.peek() {
            same.push(l.next().unwrap());
            r.next();
            c.next();
        }
        if !same.is_empty() {
            let joined = same.join(split);
            if split != "" || joined != "" {
                ret.push(Difference::Same(joined));
            }
        }

        let mut rem = Vec::new();
        while l.peek().is_some() && l.peek() != c.peek() {
            rem.push(l.next().unwrap());
        }
        if !rem.is_empty() {
            ret.push(Difference::Rem(rem.join(split)));
        }

        let mut add = Vec::new();
        while r.peek().is_some() && r.peek() != c.peek() {
            add.push(r.next().unwrap());
        }
        if !add.is_empty() {
            ret.push(Difference::Add(add.join(split)));
        }
    }

    ret
}

fn merge_without_common(orig: &str, edit: &str, split: &str) -> Vec<Difference> {
    let l = orig.split(split).collect::<Vec<&str>>();
    let r = edit.split(split).collect::<Vec<&str>>();
    let mut l_iter = l.iter();
    let mut r_iter = r.iter();
    let mut ret = Vec::new();

    while let Option::Some(ref rem) = l_iter.next() {
        ret.push(Difference::Rem(rem.to_string()));
        match r_iter.next() {
            Some(ref add) => ret.push(Difference::Add(add.to_string())),
            None => break
        }
    }

    // consume the remaining parts (if any) on left, then right
    while let Option::Some(ref rem) = l_iter.next() {
        ret.push(Difference::Rem(rem.to_string()));
    }
    while let Option::Some(ref add) = r_iter.next() {
        ret.push(Difference::Add(add.to_string()));
    }

    ret
}


#[test]
fn test_merge() {
    assert_eq!(
        merge("testa", "tost", "tst", ""),
        vec![
            Difference::Same("t".to_string()),
            Difference::Rem("e".to_string()),
            Difference::Add("o".to_string()),
            Difference::Same("st".to_string()),
            Difference::Rem("a".to_string()),
        ]
    );

    assert_eq!(
        merge("", "a", "", ""),
        vec![Difference::Add("a".to_string())]
    );

    assert_eq!(
        merge("a\nb", "a\n\nb", "a\nb", "\n"),
        vec![
            Difference::Same("a".to_string()),
            Difference::Add("".to_string()),
            Difference::Same("b".to_string()),
        ]
    );

    assert_eq!(
        merge("a\n", "c\n", "\n", "\n"),
        vec![
            Difference::Rem("a".to_string()),
            Difference::Add("c".to_string()),
            Difference::Same("".to_string()),
        ]
    );
}
