use aoc::*;

fn main() {
    let mut foo = Vec::new();
    if let Ok(lines) = read_lines("./input/p3.txt") {
        for line in lines {
            for (i, ch) in line.unwrap().chars().enumerate() {
                if foo.len() <= i {
                    foo.push(0);
                }

                foo[i] += match ch {
                    '1' => 1,
                    '0' => -1,
                    _ => panic!("this ain't binary fam"),
                };
            }
        }

        let gamma_bits = foo.iter().map(|&x| if x >= 0 { '1' } else { '0' }).collect::<String>();
        let epsilon_bits = foo.iter().map(|&x| if x >= 0 { '0' } else { '1' }).collect::<String>();

        let gamma = binstr_to_dec(gamma_bits.as_str());
        let epsilon = binstr_to_dec(epsilon_bits.as_str());

        println!("{}", gamma * epsilon)
    }
}