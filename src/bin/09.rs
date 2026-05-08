advent_of_code::solution!(9);

use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Map {
    File { id: usize, length: u8 },
    Gap(u8),
}

struct Puzzle {
    disk_map: VecDeque<Map>,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut id = 0;
        let mut disk_map = VecDeque::new();
        for byte in input.trim_ascii_end().bytes() {
            let length = byte - b'0';
            if disk_map.len() % 2 == 0 {
                disk_map.push_back(Map::File { id, length });
                id += 1;
            } else {
                disk_map.push_back(Map::Gap(length));
            }
        }
        Self { disk_map }
    }

    fn checksum(&self) -> usize {
        let mut map = self.disk_map.clone();
        let mut sum = 0;
        // The current file or gap we're filling in
        let mut head = map.pop_front();
        // The file we're filling with
        let mut filling = map.pop_back();
        map.pop_back();
        for i in 0.. {
            match head {
                Some(Map::File { id, ref mut length }) => {
                    sum += i * id;
                    *length -= 1;
                    if *length == 0 {
                        head = map.pop_front();
                        if let Some(Map::Gap(left)) = head
                            && left == 0
                        {
                            // skip over empty gaps
                            head = map.pop_front();
                        }
                    }
                }
                Some(Map::Gap(ref mut left)) => {
                    if let Some(Map::File { id, ref mut length }) = filling {
                        sum += i * id;
                        // shorten the fill file and check if it's drained
                        *length -= 1;
                        if *length == 0 {
                            // get next fill off the end of the queue
                            filling = map.pop_back();
                            map.pop_back(); // should be the preceding Gap
                        }
                    }
                    // close the gap
                    *left -= 1;
                    if *left == 0 {
                        head = map.pop_front();
                    }
                }
                None => {
                    // Drain the tail
                    if let Some(Map::File { id, ref mut length }) = filling {
                        sum += i * id;
                        *length -= 1;
                        if *length == 0 {
                            filling = None;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.checksum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
