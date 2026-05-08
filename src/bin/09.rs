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

    fn compress_checksum(&self) -> usize {
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

    fn max_id(&self) -> usize {
        if let Some(Map::File { id, .. }) = self.disk_map.back() {
            *id
        } else {
            todo!()
        }
    }

    fn defrag(&self) -> VecDeque<Map> {
        let mut map = self.disk_map.clone();
        for i in (1..=self.max_id()).rev() {
            let file_index = map
                .iter()
                .rposition(|e| matches!(e, Map::File{id,..} if i == *id))
                .unwrap();
            let Map::File { length, .. } = map.get(file_index).unwrap() else {
                panic!("was a file when we called rposition");
            };
            if let Some(gap_index) = map
                .iter()
                .position(|e| matches!(e, Map::Gap(size) if size >= length))
                && gap_index < file_index
            {
                // remove file
                let file = map.remove(file_index);
                let Some(Map::File { length, .. }) = file else {
                    panic!("file should be there")
                };

                if let Some(Map::Gap(size)) = map.get_mut(gap_index) {
                    // shrink existing gap
                    *size -= length;
                }
                // reinsert file
                map.insert(gap_index, file.expect("file"));

                // fuse gaps from where the old file was
                let right_size = if let Some(Map::Gap(right_size)) = map.get(file_index + 1) {
                    Some(*right_size)
                } else {
                    None
                };

                if right_size.is_some() {
                    map.remove(file_index + 1);
                }

                if let Some(Map::Gap(left_size)) = map.get_mut(file_index) {
                    *left_size += length;
                    *left_size += right_size.unwrap_or(0);
                }

                // Drop the last element if it's just an accumulating gap
                map.pop_back_if(|e| matches!(e, Map::Gap(_)));

                // self.draw_map(&map);
            }
        }

        map
    }

    #[allow(dead_code)]
    fn draw_map(&self, map: &VecDeque<Map>) {
        let mut map = map.clone();
        loop {
            let head = map.pop_front();
            match head {
                Some(Map::File { id, length }) => {
                    for _ in 0..length {
                        if id < 10 {
                            print!("{}", id);
                        } else {
                            print!("{} ", id);
                        }
                    }
                }
                Some(Map::Gap(size)) => {
                    for _ in 0..size {
                        print!(".");
                    }
                }
                None => {
                    println!();
                    break;
                }
            }
        }
    }

    fn defrag_checksum(&self) -> usize {
        let mut map = self.defrag();
        let mut sum: usize = 0;
        let mut index = 0;
        loop {
            let head = map.pop_front();
            match head {
                Some(Map::File { id, length }) => {
                    for _ in 0..length {
                        sum += index * id;
                        index += 1;
                    }
                }
                Some(Map::Gap(length)) => {
                    for _ in 0..length {
                        index += 1;
                    }
                }
                None => break,
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.compress_checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input);
    Some(puzzle.defrag_checksum())
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
        assert_eq!(result, Some(2858));
    }
}
