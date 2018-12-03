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
            audio: 0,
            map: convert_map_str("
                +++++++++
                ++++   F+
                ++++ ++++
                +R   ++++
                +++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "My name is Little Red.\n\n[SPACE] continue"),
                (BlockState::Red, "Every morning I go to grandma's\nhouse in the forest.\nI can use arrow keys to move,\nor [Enter] to stay."),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        1 => LevelConfig {
            audio: 0,
            map: convert_map_str("
                +++++++++
                ++++  MF+
                ++++ + ++
                ++++ + ++
                +R     ++
                +++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "Sometimes there are monsters.\nThey run as fast as me!"),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        2 => LevelConfig {
            audio: 0,
            map: convert_map_str("
                ++++++++++
                +F       +
                ++++ + + +
                ++++ + + +
                +   M+ +M+
                + ++++ +++
                +R      W+
                ++++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "The forest is mysterious\nand keeping changing..."),
            ],
            related_skill: 0,
            witch_dialog: vec![
                (BlockState::Witch, "Little Red!"),
                (BlockState::Red, "!"),
                (BlockState::Witch, "I am a witch."),
                (BlockState::Witch, "Monsters are much more\nterrifying in this forest."),
                (BlockState::Witch, "You will be eaten if caught\nby one of them!"),
                (BlockState::Red, "*nod* *nod*"),
                (BlockState::Witch, "I can teach you ice magic.\nIt can freeze one empty land\nand prevent them coming near."),
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
                    (BlockState::Witch, "Great!\nTry click on an empty land!"),
                ],
            ],
            blue_dialog: vec![],
        },

        3 => LevelConfig {
            audio: 0,
            map: convert_map_str("
                ++++++++
                +F     +
                ++++ + +
                ++++ +M+
                +  M   +
                + ++ + +
                + ++ + +
                +R   +M+
                ++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "This magic is super convinient!"),
                (BlockState::Red, "I should practice more."),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![
                vec![],
            ],
            blue_dialog: vec![],
        },

        4 => LevelConfig {
            audio: 1,
            map: convert_map_str("
                +++++++++
                +F+     +
                + + +++ +
                + + B++ +
                +     M +
                +++++++ +
                +R      +
                +++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "!"),
                (BlockState::Red, "There is a girl!\nI should help her!"),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![
                (BlockState::Blue, "Thanks!"),
                (BlockState::Red, "Be careful next time.\nThis forest is dangerous!"),
                (BlockState::Blue, "Your magic is amazing!"),
                (BlockState::Red, "Haha! A witch taught me\nthis magic several day ago."),
                (BlockState::Blue, "*heart*"),
                (BlockState::Red, "Let's go home first!"),
            ],
        },

        5 => LevelConfig {
            audio: 1,
            map: convert_map_str("
                +++++++++++
                +F     M+++
                ++++   ++++
                +++M +  +++
                +       +++
                +  + +  M++
                +  +   + ++
                +BR      W+
                +++++++++++
            "),
            initial_dialog: vec![
                (BlockState::Blue, "I feel much safer with magic!"),
                (BlockState::Red, "Well... Still, be careful.\nMonsters are becoming more."),
            ],
            related_skill: 1,
            witch_dialog: vec![
                (BlockState::Witch, "Well..."),
                (BlockState::Witch, "It is much more dangerous."),
                (BlockState::Red, "I have never seen so many!"),
                (BlockState::Witch, "If you feel dangerous,\nyou can stay in\ncurrent block with [Enter]."),
                (BlockState::Red, "I know it."),
                (BlockState::Witch, "And...\nI know a great magic.\nIt can give you sharp teeth."),
                (BlockState::Witch, "If you have sharp teeth,\nyou can eat monsters!"),
                (BlockState::Witch, "But, of cause,\nyou have to sacrifice your\noriginal teeth."),
                (BlockState::Flower, "Choose:\n\n[1] Get sharp teeth!\n[2] No, I'll find other way."),
            ],
            choices_dialog: vec![
                vec![
                    (BlockState::Red, "I want sharp teeth!"),
                    (BlockState::Witch, "OK."),
                    (BlockState::Witch, "@#^$%&^#^*&%^$*%"),
                    (BlockState::Red, "..."),
                    (BlockState::Witch, "@^$^&!!&**^%$$*"),
                    (BlockState::Red, "Feeling strange..."),
                    (BlockState::Witch, "To eat a monster,\nstep on the monster's block."),
                ],
                vec![
                    (BlockState::Red, "No, I will find another way."),
                    (BlockState::Witch, "Good luck!"),
                ],
            ],
            blue_dialog: vec![],
        },

        6 => LevelConfig {
            audio: 1,
            map: convert_map_str("
                +++++++++
                +F  M   +
                ++ + ++ +
                ++M+ ++M+
                ++ + ++ +
                +  M    +
                + ++  + +
                + ++  + +
                +  +    +
                +BR  W+ +
                +++++++++
            "),
            initial_dialog: vec![
                (BlockState::Blue, "That many?!"),
            ],
            related_skill: 2,
            witch_dialog: vec![
                (BlockState::Witch, "Well..."),
                (BlockState::Witch, "I have another magic.\nA strong one."),
                (BlockState::Red, "!"),
                (BlockState::Witch, "It casts a fire ball,\nburn one of the enemies,\nand keep the block on fire."),
                (BlockState::Witch, "But you have to sacrifice\nyour beautiful face."),
                (BlockState::Witch, "Do you want to learn it?"),
                (BlockState::Flower, "Choose:\n\n[1] Learn fire magic!\n[2] No, I'll find other way."),
            ],
            choices_dialog: vec![
                vec![
                    (BlockState::Red, "I need power!"),
                    (BlockState::Witch, "OK."),
                    (BlockState::Witch, "@#!^^%*#$@#%^$^"),
                    (BlockState::Red, "..."),
                    (BlockState::Witch, "#$^$%^&%^*^$%!%"),
                    (BlockState::Red, "..."),
                ],
                vec![
                    (BlockState::Red, "No, I will find another way."),
                    (BlockState::Witch, "Good luck!"),
                ],
            ],
            blue_dialog: vec![],
        },

        7 => LevelConfig {
            audio: 1,
            map: convert_map_str("
                +++++++++++
                +F      M +
                +++++ + + +
                +++++M  + +
                +++++++   +
                ++M++    ++
                ++ +M +   +
                ++ ++++ + +
                +BR       +
                +++++++++++
            "),
            initial_dialog: vec![
                (BlockState::Blue, "!!!"),
                (BlockState::Red, "I will protect you!"),
            ],
            related_skill: 1,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        8 => LevelConfig {
            audio: 2,
            map: convert_map_str("
                +++++++++++
                +F        +
                +++++M+++ +
                +++++ +++ +
                +++++ +++ +
                ++M  B  M +
                +++++ +++ +
                +++++ +++ +
                +++++M+++ +
                +++       +
                +R  W++++++
                +++++++++++
            "),
            initial_dialog: vec![
                (BlockState::Blue, "Help!"),
                (BlockState::Red, "Why are you there?"),
                (BlockState::Blue, "I was lost in the forest."),
                (BlockState::Blue, "Then I found monsters are\neverywhere!"),
                (BlockState::Red, "I will help with magic!"),
            ],
            related_skill: 1,
            witch_dialog: vec![
                (BlockState::Witch, "Well..."),
                (BlockState::Witch, "I have no idea now."),
                (BlockState::Witch, "Monsters are just crazy."),
                (BlockState::Witch, "If you have accepted all my\noffers before, I have a way to\nrescue you both..."),
                (BlockState::Red, "..."),
                (BlockState::Witch, "..."),
                (BlockState::Red, "..."),
                (BlockState::Witch, "..."),
                (BlockState::Unreachable, ""),
            ],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        9 => LevelConfig {
            audio: 2,
            map: convert_map_str("
                +++++++
                +R   F+
                +++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "That day,\nI lost an important friend."),
                (BlockState::Red, "Then I killed all the monsters\nwith great anger."),
                (BlockState::Red, "The forest is peaceful now..."),
                (BlockState::Red, "with my friend\nsleeping here forever."),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        10 => LevelConfig {
            audio: 2,
            map: convert_map_str("
                +++++++++++
                +F        +
                +++++M+++ +
                +++++ +++ +
                +++++ +++ +
                ++M  B  M +
                +++++ +++ +
                +++++ +++ +
                +++++M+++ +
                +++       +
                +R  W++++++
                +++++++++++
            "),
            initial_dialog: vec![
                (BlockState::Blue, "Help!"),
                (BlockState::Red, "Why are you there?"),
                (BlockState::Blue, "I was lost in the forest."),
                (BlockState::Blue, "Then I found monsters are\neverywhere!"),
                (BlockState::Red, "I will help with magic!"),
            ],
            related_skill: 1,
            witch_dialog: vec![
                (BlockState::Witch, "Well..."),
                (BlockState::Witch, "Monsters are just crazy."),
                (BlockState::Witch, "I have a magic."),
                (BlockState::Witch, "An ultimate one..."),
                (BlockState::Witch, "with great sacrifices."),
                (BlockState::Witch, "You sacrifice your consciousness,\nvanishing all other monsters."),
                (BlockState::Red, "..."),
                (BlockState::Witch, "..."),
                (BlockState::Red, "..."),
                (BlockState::Witch, "..."),
                (BlockState::Red, "I..."),
                (BlockState::Red, "accept it."),
                (BlockState::Witch, "OK."),
                (BlockState::Witch, "@%$#!#&@^&*(&^)"),
                (BlockState::Red, "..."),
                (BlockState::Witch, "&%^*$%#!@%&%$*"),
                (BlockState::Red, "..."),
                (BlockState::Witch, "@#%^%@#$&"),
                (BlockState::Witch, "%$&^&%"),
                (BlockState::Witch, "$%^"),
                (BlockState::Unreachable, ""),
            ],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        11 => LevelConfig {
            audio: 3,
            map: convert_map_str("
                ++++++++
                +     F+
                +R    M+
                ++++++++
            "),
            initial_dialog: vec![
                (BlockState::Red, "That day, when I woke up,\nI can't find Little Red\nanywhere."),
                (BlockState::Red, "Monsters were just\ndisappeared, leaving peace\nin this forest."),
                (BlockState::Red, "I will continue searching\nfor Little Red -"),
                (BlockState::Red, "the savior of the forest"),
            ],
            related_skill: 0,
            witch_dialog: vec![],
            choices_dialog: vec![],
            blue_dialog: vec![],
        },

        _ => {
            panic!();
        }
    }
}

pub fn len() -> usize {
    12
}
