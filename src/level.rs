use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use glayout::{canvas};
use glayout::tree::{TreeNodeRc};
use glayout::canvas::element::{Event, Element, Empty, Text, Image};
use glayout::canvas::element::style::{PositionType, DisplayType};
use super::{play_audio};

#[derive(Copy, Clone, PartialEq)]
pub enum BlockState {
    Empty,
    Unreachable,
    Red,
    Blue,
    Witch,
    Monster,
    Flower,
    Fire,
    Ice,
}

#[derive(Clone)]
pub struct LevelConfig {
    pub audio: i32,
    pub map: Vec<Vec<BlockState>>,
    pub initial_dialog: Vec<(BlockState, &'static str)>,
    pub witch_dialog: Vec<(BlockState, &'static str)>,
    pub blue_dialog: Vec<(BlockState, &'static str)>,
    pub choices_dialog: Vec<Vec<(BlockState, &'static str)>>,
    pub related_skill: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct RedSkills {
    pub ice: bool,
    pub bite: bool,
    pub fire: bool,
}

struct MonsterStates {
    pos: (usize, usize),
    moving: bool,
}

pub struct Level { }

const BLOCK_SIZE: f64 = 60.;

impl Level {
    fn end_level(context: &Rc<RefCell<canvas::CanvasContext>>, resource: &super::resource::Resource, num: usize, skills: RedSkills) {
        let ctx = context.clone();
        ctx.borrow_mut().root().remove(0);
        if num < super::level_config::len() {
            Self::show(context.clone(), resource.clone(), num, skills);
        } else {
            super::cover::Cover::show(context.clone(), resource.clone(), if num == 12 {
                "blue"
            } else {
                match (skills.bite, skills.fire) {
                    (false, false) => "red",
                    (false, true) => "red_with_skin",
                    (true, false) => "red_with_teeth",
                    (true, true) => "red_with_teeth_skin",
                }
            });
        }
    }

    fn update_map(resource: &super::resource::Resource, node: &TreeNodeRc<Element>, map: &Vec<Vec<BlockState>>, skills: RedSkills, num: usize) {
        for j in 0..map.len() {
            let row = &map[j];
            let row_node = node.child(j);
            for i in 0..row.len() {
                let cell = row[i].clone();
                let cell_node = row_node.child(i);
                let loader_name = match cell {
                    BlockState::Empty => "empty",
                    BlockState::Unreachable => "unreachable",
                    BlockState::Red => if num == 11 {
                            "blue"
                        } else {
                            match (skills.bite, skills.fire) {
                                (false, false) => "red",
                                (false, true) => "red_with_skin",
                                (true, false) => "red_with_teeth",
                                (true, true) => "red_with_teeth_skin",
                            }
                        },
                    BlockState::Blue => "blue",
                    BlockState::Witch => "witch",
                    BlockState::Monster => "monster",
                    BlockState::Flower => "flower",
                    BlockState::Ice => "ice",
                    BlockState::Fire => "fire",
                };
                cell_node.elem().content_mut().downcast_mut::<Image>().unwrap().set_loader(resource.image(loader_name));
            };
        }
    }

    fn show_dialog(resource: &super::resource::Resource, speaking_node: &mut TreeNodeRc<Element>, map: &Vec<Vec<BlockState>>, block_position: (usize, usize), content: &str) {
        if content.len() == 0 {
            Self::hide_dialog(speaking_node);
            return;
        }
        speaking_node.elem().style_mut().display(DisplayType::Block);
        let position = (
            (1280. - BLOCK_SIZE * map[0].len() as f64) / 2. + BLOCK_SIZE * block_position.0 as f64 + BLOCK_SIZE / 2.,
            (720. - BLOCK_SIZE * map.len() as f64) / 2. + BLOCK_SIZE * block_position.1 as f64 + BLOCK_SIZE / 2.
        );
        let height = content.split("\n").collect::<Vec<&str>>().len() as f64 * 24. + 20.;
        let width = 300.;
        let left = position.0 - 150.;
        let top = if position.1 >= 360. {
            position.1 - height - 30. - BLOCK_SIZE / 2.
        } else {
            position.1 + 30. + BLOCK_SIZE / 2.
        };
        speaking_node.elem().style_mut().left(left);
        speaking_node.elem().style_mut().top(top);
        speaking_node.elem().style_mut().width(width);
        speaking_node.elem().style_mut().height(height);
        let text_node = speaking_node.child(0);
        text_node.elem().content_mut().downcast_mut::<Text>().unwrap().set_text(content);
        let arrow_node = speaking_node.child(1);
        if position.1 >= 360. {
            arrow_node.elem().style_mut().top(height);
            arrow_node.elem().content_mut().downcast_mut::<Image>().unwrap().set_loader(resource.image("speaking_arrow"));
        } else {
            arrow_node.elem().style_mut().top(-30.);
            arrow_node.elem().content_mut().downcast_mut::<Image>().unwrap().set_loader(resource.image("speaking_arrow_revert"));
        }
    }

    fn hide_dialog(speaking_node: &mut TreeNodeRc<Element>) {
        speaking_node.elem().style_mut().left(9999.);
        speaking_node.elem().style_mut().top(9999.);
        // speaking_node.elem().style_mut().display(DisplayType::None);
    }

    pub fn show(context: Rc<RefCell<canvas::CanvasContext>>, resource: super::resource::Resource, mut num: usize, mut skills: RedSkills) {
        if skills.bite && skills.fire && num == 8 {
            num = 10;
        }

        let original_skills = skills.clone();
        let context_clone = context.clone();
        let resource = resource.clone();
        let mut level_config = super::level_config::get(num);
        let row_count = level_config.map.len();
        let col_count = level_config.map[0].len();

        // audio
        unsafe {
            play_audio(level_config.audio)
        };

        // initial layout
        let context = context_clone.clone();
        let mut ctx = context.borrow_mut();
        let cfg = ctx.canvas_config();
        let mut root = ctx.root();
        root.append(element!(&cfg, Empty {
            Empty {
                id: "map";
                position: PositionType::Absolute;
                left: (1280. - BLOCK_SIZE * col_count as f64) / 2.;
                top: (720. - BLOCK_SIZE * row_count as f64) / 2.;
            };
            Empty {
                id: "speaking";
                // display: DisplayType::None;
                position: PositionType::Absolute;
                display: DisplayType::Block;
                width: 300.;
                font_size: 16.;
                line_height: 24.;
                color: (0.3, 0.3, 0.3, 1.);
                background_color: (0.8, 0.8, 0.8, 1.);
                Text {
                    position: PositionType::Absolute;
                    left: 20.;
                    width: 260.;
                    top: 10;
                    display: DisplayType::Block;
                };
                Image {
                    position: PositionType::Absolute;
                    display: DisplayType::Block;
                    left: 135.;
                    height: 30.;
                    width: 30.;
                    set_loader(resource.image("speaking_arrow"));
                };
            };
            Empty {
                id: "fading_cover";
                position: PositionType::Absolute;
                display: DisplayType::Block;
                width: 1280.;
                height: 720.;
                opacity: 1.;
                background_color: (0.2, 0.2, 0.2, 1.);
            };
        }));
        let mut map = ctx.node_by_id("map").unwrap();
        let mut speaking = ctx.node_by_id("speaking").unwrap();
        let fading_cover = ctx.node_by_id("fading_cover").unwrap();

        // put blocks into map
        let mut flower_pos = (0, 0);
        let mut red_pos = (0, 0);
        let mut blue_pos = None;
        let mut witch_pos = None;
        let mut monster_states = vec![];
        let prev_click_pos: Rc<Cell<_>> = Rc::new(Cell::new(None));
        for j in 0..level_config.map.len() {
            let row = &level_config.map[j];
            let mut row_node = element!(&cfg, Empty {
                position: PositionType::Absolute;
                left: 0.;
                top: j as f64 * BLOCK_SIZE;
            });
            map.append(row_node.clone());
            for i in 0..row.len() {
                let prev_click_pos_clone = prev_click_pos.clone();
                row_node.append(element!(&cfg, Image {
                    position: PositionType::Absolute;
                    top: 0.;
                    left: i as f64 * BLOCK_SIZE;
                    width: BLOCK_SIZE;
                    height: BLOCK_SIZE;
                    @ "touchend" => move |_: &Event| {
                        prev_click_pos_clone.set(Some((i, j)));
                    };
                }));
                if row[i] == BlockState::Flower {
                    flower_pos = (i, j);
                }
                if row[i] == BlockState::Red {
                    red_pos = (i, j);
                }
                if row[i] == BlockState::Blue {
                    blue_pos = Some((i, j));
                }
                if row[i] == BlockState::Witch {
                    witch_pos = Some((i, j));
                }
                if row[i] == BlockState::Monster {
                    monster_states.push(MonsterStates {
                        pos: (i, j),
                        moving: false,
                    });
                }
            }
        }
        Self::update_map(&resource, &map, &level_config.map, skills, num);

        // states
        let mut level_ended = None;
        let mut current_msg: Option<(BlockState, &'static str)> = None;
        let mut current_msg_len = 0;
        let mut pending_msg = level_config.initial_dialog.clone();
        let mut witch_dialog_shown = false;
        let mut blue_dialog_shown = false;
        let mut skill_used = RedSkills {
            ice: false,
            bite: false,
            fire: false,
        };
        let mut fading_status = 0.;
        let mut cheating_enabled = false;

        let context = context_clone;
        frame!(move |_| {
            if level_ended.is_some() {
                fading_status -= 0.12;
                if fading_status > 0. {
                    fading_cover.elem().style_mut().width(1280.);
                    fading_cover.elem().style_mut().height(720.);
                    fading_cover.elem().style_mut().opacity(1. - fading_status);
                    let mut ctx = context.borrow_mut();
                    ctx.redraw();
                    return true;
                }
                let mut lv = level_ended.unwrap();
                if num == 9 && lv == 10 {
                    lv = 13;
                }
                Self::end_level(&context, &resource, lv, skills);
                return false;
            }
            if fading_status < 1. {
                fading_status += 0.08;
                if fading_status >= 1. {
                    fading_status = 1.;
                    fading_cover.elem().style_mut().width(0.);
                    fading_cover.elem().style_mut().height(0.);
                } else {
                    fading_cover.elem().style_mut().opacity(1. - fading_status);
                }
            }

            // handling keys
            let mut ctx = context.borrow_mut();
            let current_key = ctx.fetch_last_key_code();
            let effective_key = if current_key.is_down {
                Some(current_key)
            } else {
                None
            };

            // fetching clicks
            let effective_click = prev_click_pos.replace(None);

            // cheating keys!
            match effective_key {
                Some(ref key) => {
                    match key.key_code {
                        48 => {
                            cheating_enabled = true;
                            return true;
                        },
                        55 => {
                            if cheating_enabled {
                                skills.ice = true;
                                Self::update_map(&resource, &map, &level_config.map, skills, num);
                                return true;
                            }
                        },
                        56 => {
                            if cheating_enabled {
                                skills.bite = true;
                                Self::update_map(&resource, &map, &level_config.map, skills, num);
                                return true;
                            }
                        },
                        57 => {
                            if cheating_enabled {
                                skills.fire = true;
                                Self::update_map(&resource, &map, &level_config.map, skills, num);
                                return true;
                            }
                        },
                        54 => {
                            if cheating_enabled {
                                level_ended = Some(num + 1);
                                return true;
                            }
                        },
                        _ => { },
                    }
                },
                _ => { }
            }

            // check witch and blue dialogs
            if witch_pos.is_some() && (red_pos.0 as i32 - witch_pos.unwrap().0 as i32).abs() + (red_pos.1 as i32 - witch_pos.unwrap().1 as i32).abs() <= 1 {
                if !witch_dialog_shown {
                    witch_dialog_shown = true;
                    pending_msg.append(&mut level_config.witch_dialog.clone());
                }
            } else if blue_pos.is_some() && (red_pos.0 as i32 - blue_pos.unwrap().0 as i32).abs() + (red_pos.1 as i32 - blue_pos.unwrap().1 as i32).abs() <= 1 {
                if !blue_dialog_shown {
                    blue_dialog_shown = true;
                    pending_msg.append(&mut level_config.blue_dialog.clone());
                }
            }

            // dialog
            match current_msg.clone() {
                Some(msg) => {
                    if effective_key.is_some() {
                        let key_code = effective_key.unwrap().key_code;
                        if key_code == 32 && msg.0 != BlockState::Flower {
                            if current_msg_len == msg.1.len() {
                                current_msg = None;
                            } else {
                                current_msg_len = msg.1.len() - 1;
                            }
                        } else if msg.0 == BlockState::Flower {
                            let choice = key_code - 49;
                            if choice < 0 || choice >= level_config.choices_dialog.len() as i32 {
                                // do nothing
                            } else {
                                if choice == 0 {
                                    match level_config.related_skill {
                                        0 => { skills.ice = true; },
                                        1 => { skills.bite = true; },
                                        2 => { skills.fire = true; },
                                        _ => { panic!() }
                                    }
                                }
                                pending_msg.append(&mut level_config.choices_dialog[choice as usize].clone());
                                current_msg = None;
                            }
                        }
                    } else {
                        if current_msg_len < msg.1.len() {
                            current_msg_len += 1;
                            let current_msg = current_msg.clone().unwrap();
                            Self::show_dialog(&resource, &mut speaking, &level_config.map, match current_msg.0 {
                                BlockState::Red => red_pos,
                                BlockState::Flower => red_pos,
                                BlockState::Blue => blue_pos.unwrap(),
                                BlockState::Witch => witch_pos.unwrap(),
                                BlockState::Unreachable => {
                                    return true;
                                },
                                _ => panic!()
                            }, &current_msg.1[0..current_msg_len]);
                            ctx.redraw();
                        }
                    }
                    return true;
                },
                None => {
                    if pending_msg.len() > 0 {
                        let t = Some(pending_msg.remove(0));
                        current_msg = t;
                        current_msg_len = 0;
                        if current_msg.unwrap().0 == BlockState::Unreachable {
                            level_ended = Some(num + 1);
                            return true;
                        }
                        return true;
                    }
                    Self::hide_dialog(&mut speaking);
                    Self::update_map(&resource, &map, &level_config.map, skills, num);
                }
            }

            // handling click action
            if effective_click.is_some() {
                let click_pos = effective_click.unwrap();
                let s = level_config.map[click_pos.1][click_pos.0].clone();
                if s == BlockState::Empty {
                    if skills.ice {
                        if skill_used.ice {
                            pending_msg.append(&mut vec![(BlockState::Red, "I have already used the\nice magic...")]);
                        } else {
                            pending_msg.append(&mut vec![(BlockState::Red, "Ice wall !!!")]);
                            level_config.map[click_pos.1][click_pos.0] = BlockState::Ice;
                            skill_used.ice = true;
                            Self::update_map(&resource, &map, &level_config.map, skills, num);
                        }
                    }
                } else if s == BlockState::Monster {
                    if skills.fire {
                        if skill_used.fire {
                            pending_msg.append(&mut vec![(BlockState::Red, "I have already used the\nfire magic...")]);
                        } else {
                            pending_msg.append(&mut vec![(BlockState::Red, "Fire ball !!!")]);
                            level_config.map[click_pos.1][click_pos.0] = BlockState::Fire;
                            for i in 0..monster_states.len() {
                                if monster_states[i].pos == click_pos {
                                    monster_states.remove(i);
                                    break;
                                }
                            }
                            skill_used.fire = true;
                            Self::update_map(&resource, &map, &level_config.map, skills, num);
                        }
                    }
                }
            }

            // handling key action
            let direction: (i32, i32) = match effective_key {
                Some(ref key) => {
                    match key.key_code {
                        37 | 97 => {
                            (-1, 0) // left
                        },
                        38 | 119 => {
                            (0, -1) // up
                        },
                        39 | 100 => {
                            (1, 0) // right
                        },
                        40 | 115 => {
                            (0, 1) // down
                        }
                        13 => {
                            (0, 0) // enter
                        },
                        114 | 82 => {
                            level_ended = Some(num);
                            skills = original_skills.clone();
                            return true;
                        },
                        _ => {
                            (-1, -1)
                        }
                    }
                },
                None => {
                    (-1, -1)
                }
            };
            if direction == (-1, -1) { return true };

            // move red and blue
            if direction != (0, 0) {
                let move_target = ((red_pos.0 as i32 + direction.0) as usize, (red_pos.1 as i32 + direction.1) as usize);
                let target_type = level_config.map[move_target.1][move_target.0].clone();
                let mut move_success = true;
                if target_type != BlockState::Empty && target_type != BlockState::Flower {
                    if target_type == BlockState::Monster || num == 11 {
                        if skills.bite {
                            if skill_used.bite {
                                pending_msg.append(&mut vec![(BlockState::Red, "I am already full.\nI can't eat monsters now.")]);
                                move_success = false;
                            } else {
                                pending_msg.append(&mut vec![(BlockState::Red, "*yum* *yum*")]);
                                for i in 0..monster_states.len() {
                                    if monster_states[i].pos == move_target {
                                        monster_states.remove(i);
                                        break;
                                    }
                                }
                                skill_used.bite = true;
                            }
                        } else {
                            pending_msg.append(&mut vec![(BlockState::Red, "Monster... X_X")]);
                            level_ended = Some(num);
                            skills = original_skills.clone();
                            move_success = false;
                        }
                    } else {
                        move_success = false;
                    }
                }
                if !move_success {
                    return true;
                }
                level_config.map[red_pos.1][red_pos.0] = BlockState::Empty;
                level_config.map[move_target.1][move_target.0] = BlockState::Red;
                if blue_pos.is_some() {
                    let blue_pos_u = blue_pos.clone().unwrap();
                    if (red_pos.0 as i32 - blue_pos_u.0 as i32).abs() + (red_pos.1 as i32 - blue_pos_u.1 as i32).abs() <= 1 {
                        level_config.map[blue_pos_u.1][blue_pos_u.0] = BlockState::Empty;
                        level_config.map[red_pos.1][red_pos.0] = BlockState::Blue;
                        blue_pos = Some(red_pos);
                    }
                }
                red_pos = move_target;
            }

            // check win
            if red_pos == flower_pos {
                level_ended = Some(num + 1);
                return true;
            }

            // monster moves
            for state in monster_states.iter_mut() {
                if !state.moving {
                    if (red_pos.0 as i32 - state.pos.0 as i32).abs() + (red_pos.1 as i32 - state.pos.1 as i32).abs() <= 4 {
                        state.moving = true
                    } else if blue_pos.is_some() && ((blue_pos.unwrap().0 as i32 - state.pos.0 as i32).abs() + (blue_pos.unwrap().1 as i32 - state.pos.1 as i32).abs() <= 4) {
                        state.moving = true
                    } else {
                        continue
                    }
                }
                // find shortest path
                let mut ideal_direction = (0, 0);
                let mut queue = VecDeque::new();
                let mut direction = HashMap::new();
                queue.push_back(state.pos);
                direction.insert(state.pos, None);
                while queue.len() > 0 {
                    let pos = queue.pop_front();
                    if pos.is_none() { break };
                    let pos = pos.unwrap();
                    for test_direction in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
                        let move_target = ((pos.0 as i32 + test_direction.0) as usize, (pos.1 as i32 + test_direction.1) as usize);
                        let target_type = level_config.map[move_target.1][move_target.0].clone();
                        let t = direction[&pos];
                        let ideal_dir = match t {
                            Some(x) => Some(x),
                            None => Some(test_direction.clone()),
                        };
                        if target_type == BlockState::Red || target_type == BlockState::Blue {
                            ideal_direction = ideal_dir.unwrap();
                            queue.truncate(0);
                            break;
                        } else if target_type == BlockState::Empty || target_type == BlockState::Ice || target_type == BlockState::Monster {
                            if !direction.contains_key(&move_target) {
                                queue.push_back(move_target);
                                direction.insert(move_target, ideal_dir);
                            }
                        }
                    }
                }
                // move
                let new_pos = (state.pos.0 as i32 + ideal_direction.0, state.pos.1 as i32 + ideal_direction.1);
                let target_type = level_config.map[new_pos.1 as usize][new_pos.0 as usize];
                if target_type == BlockState::Ice || target_type == BlockState::Monster || (num == 11 && target_type == BlockState::Red) {
                    // do nothing
                } else {
                    level_config.map[state.pos.1][state.pos.0] = BlockState::Empty;
                    state.pos.0 = new_pos.0 as usize;
                    state.pos.1 = new_pos.1 as usize;
                    level_config.map[state.pos.1][state.pos.0] = BlockState::Monster;
                }
                // check lose
                if red_pos == state.pos || blue_pos == Some(state.pos) {
                    level_ended = Some(num);
                    skills = original_skills.clone();
                }
            }

            Self::update_map(&resource, &map, &level_config.map, skills, num);
            ctx.redraw();
            return true;
        });
    }
}
