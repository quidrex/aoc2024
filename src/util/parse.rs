#[macro_export]
macro_rules! parse {
    ([$c: tt $delim: literal]) => {
        |s: &str| s.split($delim).map(parse!($c)).collect::<Vec<_>>()
    };
    ([$c: tt $delim: literal $n: literal]) => {
        |s: &str| {
            TryInto::<[_; $n]>::try_into(s.split($delim).map(parse!($c)).collect::<Vec<_>>()).unwrap()
        }
    };
    (($l: tt $delim: literal $r: tt)) => {
        |s: &str| {
            let (l, r) = s.split_once($delim).unwrap();
            (parse!($l)(l), parse!($r)(r))
        }
    };
    (i32) => {
        |s: &str| s.parse::<i32>().unwrap()
    };
    (i64) => {
        |s: &str| s.parse::<i64>().unwrap()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(vec![1, 2, 3], parse!([i64 ","])("1,2,3"));
        assert_eq!((1, vec![2, 3]), parse!((i64 ";" [i32 ","]))("1;2,3"));
        assert_eq!([vec![1], vec![2, 3]], parse!([[i64 ","] ";" 2])("1;2,3"));
        assert_eq!(vec![[1, 2], [3, 4]], parse!([[i64 "," 2] ";"])("1,2;3,4"));
    }
}
