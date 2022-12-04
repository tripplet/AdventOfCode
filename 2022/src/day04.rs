use std::str::FromStr;

type ParseResult = Vec<(Section, Section)>;

#[derive(Debug, PartialEq)]
pub struct Section {
    start: u16,
    end: u16,
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let sections = line.split_once(",").unwrap();
            (sections.0.parse().unwrap(), sections.1.parse().unwrap())
        })
        .collect()
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once("-").ok_or("Missing delimiter")?;

        Ok(Section {
            start: parts.0.parse().map_err(|err| format!("{}", err))?,
            end: parts.1.parse().map_err(|err| format!("{}", err))?,
        })
    }
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end) || (other.end >= self.start && other.end <= self.end)
    }

    fn overlaps2(&self, other: &Self) -> bool {
        (other.start <= self.end && other.start >= self.start) || (other.end >= self.start && other.start <= self.start)
    }

    fn overlaps3(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

pub fn part1(input: &ParseResult) -> usize {
    input.iter().filter(|&(a, b)| a.contains(b) || b.contains(a)).count()
}

pub fn part2(input: &ParseResult) -> usize {
    dbg!(input
        .iter()
        .filter(|&(a, b)| { a.overlaps2(b) != a.overlaps(b) })
        .map(|(a, b)| (a, b, a.overlaps2(b), a.overlaps(b)))
        .collect::<Vec<_>>());

    input.iter().filter(|&(a, b)| a.overlaps3(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! section {
        ($start:expr, $end:expr) => {
            &Section {
                start: $start,
                end: $end,
            }
        };
        ($str:expr) => {{
            let sections = $str.split_once(",").unwrap();
            (
                &sections.0.parse::<Section>().unwrap(),
                &sections.1.parse::<Section>().unwrap(),
            )
        }};
    }

    #[test]
    fn test_parse() {
        assert!("12-46".parse::<Section>().unwrap() == Section { start: 12, end: 46 });
    }

    #[test]
    fn test_part2_example() {
        let input = parse_input(
            "5-7,7-9\n\
             2-8,3-7\n\
             6-6,4-6\n\
             2-6,4-8",
        );

        assert!(part2(&input) == 4);
    }

    #[test]
    fn test_overlap() {
        assert!(!section![12, 14].overlaps(section![15, 2]));
        assert!(section![1, 14].overlaps(section![15, 2]));
        assert!(section!("7-8,7-43").0.overlaps(section!("7-8,7-43").1));
    }
}
