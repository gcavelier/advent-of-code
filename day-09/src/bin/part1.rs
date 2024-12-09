use itertools::Itertools;
use tracing::{debug, error, info, instrument};

#[derive(Debug, Clone)]
struct FileBlock {
    file_id: Option<u32>,
}

#[instrument(skip_all)]
fn main() {
    tracing_subscriber::fmt::init();

    let input = include_str!("./input.txt");
    let output = part1(input);
    info!(output);
}

#[instrument(skip_all)]
fn reord(blocks: &mut Vec<FileBlock>) {
    for idx in 0..blocks.len() {
        if blocks[idx].file_id.is_none() {
            let idx_to_swap = blocks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.file_id.is_some())
                .map(|(i, _)| i)
                .rev()
                .next()
                .unwrap();
            if idx_to_swap > idx {
                blocks.swap(idx, idx_to_swap);
            }
        }
    }
}

#[instrument(skip_all)]
fn part1(input: &str) -> usize {
    let mut file_len = true;
    let mut idx: u32 = 0;
    let mut blocks: Vec<FileBlock> = input
        .chars()
        .filter(|c| *c != '\n')
        .flat_map(|c| {
            let capa = c.to_digit(10).unwrap() as usize;
            let mut blocks = Vec::with_capacity(capa);
            let file_block = FileBlock { file_id: Some(idx) };
            if file_len {
                // file space length
                for _ in 0..capa {
                    blocks.push(file_block.clone());
                }
                idx += 1;
            } else {
                // free space length
                for _ in 0..capa {
                    blocks.push(FileBlock { file_id: None });
                }
            }
            file_len = !file_len;
            blocks
        })
        .collect();

    //dbg!(&blocks);
    reord(&mut blocks);
    //dbg!(&blocks);

    // checksum
    blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.file_id.is_some())
        //.inspect(|(idx, c)| info!("{c} {}", **c as isize))
        .map(|(idx, b)| idx * (b.file_id.unwrap() as usize))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        let result = part1("2333133121414131402");
        //let result = part1("12345");
        assert_eq!(result, 1928);
    }
}
