use super::load_file::load_file;

fn parse(x: u8) -> usize {
    assert!(x >= b'0' && x <= b'9');
    (x - b'0') as usize
}

pub fn part1(input: String) -> usize {
    let input = input.trim().as_bytes();

    let mut i: usize = 0;
    let mut j: usize = if (input.len() - 1) % 2 == 0 {
        input.len() - 1
    } else {
        input.len() - 2
    };
    let mut j_count = parse(input[j]);

    let mut files_index: usize = 0;
    let mut result: usize = 0;
    while i < j {
        let id = i / 2;
        let count = parse(input[i]);
        let new_files_index = files_index + count as usize;
        while files_index < new_files_index {
            result += id * files_index;
            files_index += 1;
        }

        i += 1;
        let mut free_space = parse(input[i]);
        i += 1;

        while free_space > 0 {
            let count = free_space.min(j_count);
            j_count -= count;
            free_space -= count;
            let id = j / 2;
            let new_files_index = files_index + count as usize;
            while files_index < new_files_index {
                result += id * files_index;
                files_index += 1;
            }

            if j_count == 0 {
                j -= 2;
                j_count = parse(input[j]);
            }

            if j < i {
                break;
            }
        }
    }

    let id = j / 2;
    while j_count > 0 {
        result += id * files_index;
        files_index += 1;
        j_count -= 1;
    }
    assert!(j_count == 0);
    result
}

struct MemoryChunk {
    start: usize,
    size: usize,
}

struct File {
    id: usize,
    memory: MemoryChunk,
}

pub fn part2(input: String) -> usize {
    let input = input.trim().as_bytes();

    let free_space_chunks_count: usize = input.len() / 2;
    let files_count: usize = input.len() - free_space_chunks_count;

    let mut files = Vec::<File>::with_capacity(files_count);
    let mut free_space_chunks = Vec::<MemoryChunk>::with_capacity(free_space_chunks_count);

    let mut i: usize = 0;
    let mut memory_index: usize = 0;
    while i < input.len() {
        {
            let size = parse(input[i]);
            files.push(File {
                id: i / 2,
                memory: MemoryChunk {
                    start: memory_index,
                    size,
                },
            });
            memory_index += size;
        }

        i += 1;
        if i >= input.len() {
            break;
        }

        {
            let size = parse(input[i]);
            free_space_chunks.push(MemoryChunk {
                start: memory_index,
                size,
            });
            memory_index += size;
        }

        i += 1;
    }

    for file in files.iter_mut().rev() {
        if free_space_chunks.is_empty() || file.memory.start < free_space_chunks[0].start {
            break;
        }

        for i in 0..free_space_chunks.len() {
            let chunk = &mut free_space_chunks[i];
            if chunk.start > file.memory.start {
                break;
            }

            if chunk.size >= file.memory.size {
                file.memory.start = chunk.start;
                if chunk.size == file.memory.size {
                    free_space_chunks.remove(i);
                } else {
                    chunk.start += file.memory.size;
                    chunk.size -= file.memory.size;
                }
                break;
            }
        }
    }

    files
        .iter()
        .map(|file| {
            let mut value: usize = 0;
            for i in file.memory.start..file.memory.start + file.memory.size {
                value += file.id * i;
            }
            value
        })
        .sum()
}

pub fn test_input() -> String {
    load_file("res/day_09_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_09_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 1928;
    static PART1_EXPECTED_RESULT: usize = 6398252054886;
    static PART2_TEST_EXPECTED_RESULT: usize = 2858;
    static PART2_EXPECTED_RESULT: usize = 6415666220005;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input() {
        assert_eq!(part2(test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
