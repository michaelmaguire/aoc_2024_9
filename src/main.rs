use std::fs::File;
use std::io::{BufReader, Read};

const EMPTY: i64 = -1;

fn print_disk_drive(disk_drive: &Vec<i64>) {
    for &block in disk_drive {
        if block == EMPTY {
            print!(".");
        } else {
            print!("{}", block);
        }
    }
    println!();
}


fn part_1_main() {
    println!("Hello, aoc_2024_9!");
    let file_path = "./src/input.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    
    let mut disk_drive: Vec<i64> = Vec::new();

    let mut reading_block = true;
    let mut block_id: i64 = 0;
    for byte in reader.bytes() {
        let c = byte.expect("Failed to read byte") as char;
        if c.is_digit(10) {
            let digit = c.to_digit(10).expect("Failed to convert to digit") as i64;

            //println!("Digit: {}, block_id {block_id}, Reading block: {}", digit, reading_block);

            if reading_block {
                for _i in 0..digit {
                    disk_drive.push(block_id);
                }
                block_id += 1;
            } else {
                // Reading free space.
                for _i in 0..digit {
                    disk_drive.push(EMPTY);
                }
            }
            reading_block = !reading_block;
        }
    }

    print!("Disk drive: ");
    print_disk_drive(&disk_drive);


    while let Some(block) = disk_drive.pop() {
        if let Some(pos) = disk_drive.iter().position(|&x| x == EMPTY) {
            disk_drive[pos] = block;
        } else {
            // If no EMPTY slot is found, push the block back to the end
            disk_drive.push(block);
            break;
        }
    }

    print!("Disk drive after compaction: ");
    print_disk_drive(&disk_drive);

    let mut checksum: i64 = 0;
    for i in 0..disk_drive.len() {
        if disk_drive[i] == EMPTY {
            continue;
        }
        checksum += (i as i64) * disk_drive[i];
    }

    println!("Checksum: {}", checksum);

}


fn print_disk_drive_part_2(disk_drive: &Vec<Block>) {
    for block in disk_drive {
        if block.id == EMPTY {
            print!("[");
            for _i in 0..block.size {
                print!(".");
            }
            print!("]");
        } else {
            print!("[");
            for _i in 0..block.size {
                print!("{}", block.id);
            }
            print!("]");
        }
    }
    println!();
}




struct Block {
    id: i64,
    size: i64,
}
impl Block {
    fn new(id: i64, size: i64) -> Self {
        Block { id, size }
    }
}

fn checksum_part_2( disk_drive: Vec<Block>) -> i64 {
    let mut disk_drive_virtual_index = 0;
    let mut checksum: i64 = 0;
    for i in 0..disk_drive.len() {
        if disk_drive[i].id == EMPTY {
            disk_drive_virtual_index += disk_drive[i].size;
        } else {
            for _ in 0..disk_drive[i].size {
                checksum += disk_drive_virtual_index * disk_drive[i].id;
                disk_drive_virtual_index += 1;
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_part_2() {
        let disk_drive = vec![
            Block::new(1, 3),
            Block::new(EMPTY, 2),
            Block::new(2, 1),
            Block::new(EMPTY, 1),
            Block::new(3, 2),
        ];
        assert_eq!(checksum_part_2(disk_drive), 1*0 + 1*1 + 1*2 + 2*4 + 2*5);
    }

    #[test]
    fn test_checksum_part_2_empty() {
        let disk_drive = vec![
            Block::new(EMPTY, 3),
            Block::new(EMPTY, 2),
        ];
        assert_eq!(checksum_part_2(disk_drive), 0);
    }

    #[test]
    fn test_checksum_part_2_single_block() {
        let disk_drive = vec![
            Block::new(1, 5),
        ];
        assert_eq!(checksum_part_2(disk_drive), 1*0 + 1*1 + 1*2 + 1*3 + 1*4);
    }
}

fn set_size(mut disk_drive: Vec<Block>, block_index: usize, new_size: i64) {
    disk_drive[block_index].size = new_size;
}

fn main() {
    println!("Hello, aoc_2024_9!");
    let file_path = "./src/input.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    
    let mut disk_drive: Vec<Block> = Vec::new();

    let mut reading_block = true;
    let mut block_id: i64 = 0;
    for byte in reader.bytes() {
        let c = byte.expect("Failed to read byte") as char;
        if c.is_digit(10) {
            let digit = c.to_digit(10).expect("Failed to convert to digit") as i64;

            //println!("Digit: {}, block_id {block_id}, Reading block: {}", digit, reading_block);

            if reading_block {
                disk_drive.push(Block::new(block_id, digit));
                block_id += 1;
            } else {
                // Reading free space.
                if digit > 0 {
                    disk_drive.push(Block::new(EMPTY, digit));
                }
            }
            reading_block = !reading_block;
        }
    }

    print!("Disk drive: ");
    print_disk_drive_part_2(&disk_drive);

    let mut block_to_move_index = disk_drive.len() - 1;
    while block_to_move_index > 0 {
        println!("Block: block_to_move_index {}: id {} size {}", block_to_move_index, disk_drive[block_to_move_index].id, disk_drive[block_to_move_index].size);
        if disk_drive[block_to_move_index].id != EMPTY {
            for empty_block_index in 0..block_to_move_index {
                if disk_drive[empty_block_index].id == EMPTY {
                    let block_to_move_size = disk_drive[block_to_move_index].size;
                    let empty_block_size = disk_drive[empty_block_index].size;
                    //println!("Empty block at position {}: size {}", pos, empty_block.size);
                    // If the block is the same size as the EMPTY slot, just replace it
                    if disk_drive[empty_block_index].size == block_to_move_size {
                        disk_drive[empty_block_index].id = disk_drive[block_to_move_index].id;
                        disk_drive[block_to_move_index].id = EMPTY;
                        break;
                    } else if empty_block_size > block_to_move_size {
                        disk_drive[empty_block_index].size -= block_to_move_size;
                        disk_drive.insert(empty_block_index, Block::new(disk_drive[block_to_move_index].id, disk_drive[block_to_move_index].size));
                        block_to_move_index += 1;
                        disk_drive[block_to_move_index].id = EMPTY;
                        //print_disk_drive_part_2(&disk_drive);
                        break;
                    } else {
                        // If the EMPTY slot is smaller than the block, continue to the next EMPTY slot
                        continue;
                    }
                }
            }
        }
        block_to_move_index -= 1;
    }

    print!("Disk drive after compaction: ");
    print_disk_drive_part_2(&disk_drive);

    println!("Checksum: {}", checksum_part_2(disk_drive));

}
