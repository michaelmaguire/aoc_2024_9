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


fn main() {
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
        checksum += (i as i64) * (disk_drive[i] as i64);
    }

    println!("Checksum: {}", checksum);

}
