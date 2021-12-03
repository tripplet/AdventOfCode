pub fn main() {
    let input = include_str!("../input/2021/day3.txt");
    
    let (numbers, number_of_bits) = parse(input);

    println!("Part1: {}", part1(&numbers, number_of_bits));
    println!("Part2: {}", part2(&numbers, number_of_bits));
}

fn parse(input: &str) -> (Vec<u32>, u16) {
    (input.trim()
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect(), 
        input.lines().next().unwrap().len() as u16)
}

fn part1(reports: &[u32], number_of_bits: u16) -> i32 {
    let pivot = (reports.len() as f32 / 2.0).ceil() as u32;
    let bits = count_bits(reports, 0, number_of_bits as usize);

    // take all bit counts > len/2 and build the number using bit shifts from them
    let gamma_rate = bits.iter().enumerate().filter(|b| *b.1 >= pivot).fold(0, |acc, (idx, _)| acc + (1<<idx));

    // epsilon is just the bit inverse of gamma
    let epsilon_rate = !gamma_rate & ((1<<number_of_bits)-1);
    gamma_rate * epsilon_rate
}

fn part2(reports: &[u32], number_of_bits: u16) -> i32 {
    let oxygen = find_number_part2(reports, number_of_bits, false);
    let co2 = find_number_part2(reports, number_of_bits, true);
    (oxygen * co2) as i32
}

fn find_number_part2(reports: &[u32], number_of_bits: u16, keep_less: bool) -> u32 {
    let mut numbers: Vec<u32> = reports.to_vec();

    for idx in (0..number_of_bits as usize).rev() {
        let bits = count_bits(&numbers, idx, idx + 1);
        let pivot = (numbers.len() as f32 / 2.0).ceil() as u32;

        if (!keep_less && bits[idx] >= pivot) || (keep_less && bits[idx] < pivot) {
            numbers = numbers.iter().filter(|&number| number & (1<<idx) != 0).cloned().collect();
        }
        else {
            numbers = numbers.iter().filter(|&number| number & (1<<idx) == 0).cloned().collect();
        }

        if numbers.len() == 1 {
            break
        }
    }

    numbers[0]
}

fn count_bits(numbers: &[u32], start_at: usize, number_of_bits: usize) -> Vec<u32> {
    let mut bits: Vec<u32> = vec![0; number_of_bits];
    
    for bit_pos in start_at..number_of_bits {
        for report in numbers {            
            bits[bit_pos] += ((report & (1<<bit_pos)) >> bit_pos) as u32;      
        }
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_part1() {
        let (numbers, number_of_bits) = parse(EXAMPLE_DATA);
        assert_eq!(198, part1(&numbers, number_of_bits));
    }

    #[test]
    fn test_part2() {
        let (numbers, number_of_bits) = parse(EXAMPLE_DATA);
        assert_eq!(230, part2(&numbers, number_of_bits));
    }
}
