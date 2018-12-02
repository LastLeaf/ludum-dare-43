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
                +R    M+
                ++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "My name is Little Red\n\n[SPACE] continue"),
                (BlockState::Red, "Every morning I go to grandma's\nhouse in the forest.\nI can use WASD or arrow keys\nto move."),
            ],
            related_skill: 0,
            witch_dialog: vec![
                (BlockState::Witch, "Little Red!"),
                (BlockState::Red, "!"),
                (BlockState::Witch, "I am a witch.\nMonsters are much more\nTerrifying in this forest."),
                (BlockState::Witch, "You will be eaten if caught\nby one of them!"),
                (BlockState::Red, "*nod* *nod*"),
                (BlockState::Witch, "I can teach you ice magic.\nIt can block one and\nprevent them coming near."),
                (BlockState::Witch, "But you have to sacrifice\nyour beautiful ears."),
                (BlockState::Flower, "Number key to continue:\n\n[1] Learn ice magic!"),
            ],
            choices_dialog: vec![
                vec![
                    (BlockState::Red, "I'm always riding hood.\nI do not need beautiful ears!"),
                    (BlockState::Red, "Please teach me ice magic!"),
                    (BlockState::Witch, "%$^&@##!!*%*^"),
                    (BlockState::Red, "..."),
                    (BlockState::Witch, "!@#%#@$%^*^%&&*"),
                    (BlockState::Red, "..."),
                    (BlockState::Red, "I got it!"),
                    (BlockState::Witch, "Great! Try click an empty land!"),
                ],
            ],
            blue_dialog: vec![],
        },
        _ => {
            panic!();
        }
    }
}

pub fn len() -> usize {
    1
}
