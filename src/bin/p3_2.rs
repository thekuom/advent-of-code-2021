use std::collections::HashMap;
use aoc::*;

fn main() {
    if let Ok(lines) = read_lines("./input/p3.txt") {
        let mut foo = Vec::new();

        for line in lines {
            let line = line.unwrap();
            for (i, ch) in line.chars().enumerate() {
                if foo.len() <= i {
                    foo.push(HashMap::from([
                        ('0', Vec::new()),
                        ('1', Vec::new()),
                    ]));
                }

                foo[i].get_mut(&ch).unwrap().push(line.clone());
            }
        }

        let o2_scrubber = get_scrubber(&foo, true);
        let co2_scrubber = get_scrubber(&foo, false);

        println!("{}", binstr_to_dec(o2_scrubber[0].as_str()) * binstr_to_dec(co2_scrubber[0].as_str()));
    }
}

fn get_scrubber(foo: &Vec<HashMap<char, Vec<String>>>, is_o2: bool) -> Vec<String> {
    foo.iter()
       .fold(Vec::new(), |r, item| {
           let current_len = r.len();

           let zeroes = get_subset(&r, &item[&'0'], current_len);
           let ones = get_subset(&r, &item[&'1'], current_len);

           match (is_o2, ones.len(), zeroes.len()) {
               (true, x, y) if x >= y => { ones }
               (true, _, _) => { zeroes }
               (false, x, y) if x >= y => { zeroes }
               (false, _, _) => { ones }
           }
       })
}

fn get_subset<'a>(current_set: &'a Vec<String>, full_set: &'a Vec<String>, current_len: usize) -> Vec<String> {
    if current_len == 0 {
        full_set.clone()
    } else if current_len == 1 {
        current_set.clone()
    } else {
        current_set.clone().into_iter().filter(|x| full_set.contains(x)).collect()
    }
}