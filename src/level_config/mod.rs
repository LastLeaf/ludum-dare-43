use super::level::{LevelConfig, BlockState};

fn convert_map_str(s: &'static str) -> Vec<Vec<BlockState>> {
    let mut rows = vec![];
    for line in s.split('\n') {
        let mut cols = vec![];
        for char in line.chars() {
            match char {
                ' ' => {
                    if cols.len() > 0 {
                        cols.push(BlockState::Empty);
                    }
                },
                '+' => { cols.push(BlockState::Unreachable) },
                'R' => { cols.push(BlockState::Red) },
                'B' => { cols.push(BlockState::Blue) },
                'W' => { cols.push(BlockState::Witch) },
                'M' => { cols.push(BlockState::Monster) },
                'F' => { cols.push(BlockState::Flower) },
                _ => { },
            }
        }
        if cols.len() > 0 {
            rows.push(cols);
        }
    }
    rows
}

pub fn get(num: usize) -> LevelConfig {
    match num {
        0 => LevelConfig {
            map: convert_map_str("
                ++++++++
                +F    M+
                +++ ++++
                + W ++++
                +++ ++++
                +R     +
                ++++++++
            ")
        },
        _ => {
            panic!();
        }
    }
}

pub fn len() -> usize {
    1
}
