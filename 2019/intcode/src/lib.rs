pub fn run(result: &mut Vec<usize>) -> Option<usize> {
    let mut pos : usize = 0;
    while result[pos] != 99 {
        let op = result[pos];
        match op {
            1 => {
                let res = result[result[pos + 1]] + result[result[pos + 2]];
                let p = result[pos + 3];
                result[p] = res;
            },
            2 => {
                let res = result[result[pos + 1]] * result[result[pos + 2]];
                let p = result[pos + 3];
                result[p] = res;
            }
            _ => {
                return None
            }
        }
        pos += 4;
    }
    Some(result[0])
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(run(&mut input), Some(3500));
    }
}
