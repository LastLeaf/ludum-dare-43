use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};
use std::collections::{HashMap, VecDeque};
use glayout;
use glayout::{canvas};
use glayout::tree::{TreeNodeRc};
use glayout::canvas::element::{Element, Empty, Text, Image};
use glayout::canvas::element::style::{PositionType, DisplayType};
use super::{play_audio};

#[derive(Clone, PartialEq)]
pub enum BlockState {
    Empty,
    Unreachable,
    Red,
    Blue,
    Witch,
    Monster,
    Flower,
}

#[derive(Clone)]
pub struct LevelConfig {
    pub map: Vec<Vec<BlockState>>,
}

struct MonsterStates {
    pos: (usize, usize),
    moving: bool,
}

pub struct Level { }

const BLOCK_SIZE: f64 = 60.;

impl Level {
    fn end_level(context: &Rc<RefCell<canvas::CanvasContext>>, resource: &super::resource::Resource, num: usize) {
        let ctx = context.clone();
        ctx.borrow_mut().root().remove(0);
        if num < super::level_config::len() {
            Self::show(context.clone(), resource.clone(), num)
        } else {
            super::cover::Cover::show(context.clone(), resource.clone(), "blue");
        }
    }

    fn update_map(resource: &super::resource::Resource, node: &TreeNodeRc<Element>, map: &Vec<Vec<BlockState>>) {
        for j in 0..map.len() {
            let row = &map[j];
            let row_node = node.child(j);
            for i in 0..row.len() {
                let cell = row[i].clone();
                let cell_node = row_node.child(i);
                let loader_name = match cell {
                    BlockState::Empty => "empty",
                    BlockState::Unreachable => "unreachable",
                    BlockState::Red => "red",
                    BlockState::Blue => "blue",
                    BlockState::Witch => "witch",
                    BlockState::Monster => "monster",
                    BlockState::Flower => "flower",
                };
                cell_node.elem().content_mut().downcast_mut::<Image>().unwrap().set_loader(resource.image(loader_name));
            };
        }
    }

    pub fn show(context: Rc<RefCell<canvas::CanvasContext>>, resource: super::resource::Resource, num: usize) {
        let context_clone = context.clone();
        let resource = resource.clone();
        let mut level_config = super::level_config::get(num);
        let row_count = level_config.map.len();
        let col_count = level_config.map[0].len();

        // initial layout
        let context = context_clone.clone();
        let mut ctx = context.borrow_mut();
        let cfg = ctx.canvas_config();
        let mut root = ctx.root();
        root.append(element!(&cfg, Empty {
            Empty {
                id: "map";
                position: PositionType::Absolute;
                left: (1280. - BLOCK_SIZE * row_count as f64) / 2.;
                top: (720. - BLOCK_SIZE * col_count as f64) / 2.;
            };
        }));
        let mut map = ctx.node_by_id("map").unwrap();

        // put blocks into map
        let mut flower_pos = (0, 0);
        let mut red_pos = (0, 0);
        let mut blue_pos = None;
        let mut monster_states = vec![];
        for j in 0..level_config.map.len() {
            let row = &level_config.map[j];
            let mut row_node = element!(&cfg, Empty {
                position: PositionType::Absolute;
                left: 0.;
                top: j as f64 * BLOCK_SIZE;
            });
            map.append(row_node.clone());
            for i in 0..row.len() {
                row_node.append(element!(&cfg, Image {
                    position: PositionType::Absolute;
                    top: 0.;
                    left: i as f64 * BLOCK_SIZE;
                    width: BLOCK_SIZE;
                    height: BLOCK_SIZE;
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
                if row[i] == BlockState::Monster {
                    monster_states.push(MonsterStates {
                        pos: (i, j),
                        moving: false,
                    });
                }
            }
        }
        Self::update_map(&resource, &map, &level_config.map);

        // states
        let mut level_ended = None;

        let context = context_clone;
        frame!(move |_| {
            if level_ended.is_some() {
                Self::end_level(&context, &resource, level_ended.unwrap());
                return false;
            }

            // handling keys
            let mut ctx = context.borrow_mut();
            let current_key = ctx.fetch_last_key_code();
            let effective_key = if current_key.is_down {
                Some(current_key)
            } else {
                None
            };

            // handling key action
            let direction: (i32, i32) = match effective_key {
                Some(ref key) => {
                    println!("key: {:?}", key.key_code);
                    match key.key_code {
                        37 => {
                            (-1, 0) // left
                        },
                        38 => {
                            (0, -1) // up
                        },
                        39 => {
                            (1, 0) // right
                        },
                        40 => {
                            (0, 1) // down
                        }
                        _ => {
                            (0, 0) // space
                        },
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
                if target_type != BlockState::Empty && target_type != BlockState::Flower {
                    return true;
                }
                level_config.map[red_pos.1][red_pos.0] = BlockState::Empty;
                level_config.map[move_target.1][move_target.0] = BlockState::Red;
                if blue_pos.is_some() {
                    level_config.map[blue_pos.unwrap().1][blue_pos.unwrap().0] = BlockState::Empty;
                    level_config.map[red_pos.1][red_pos.0] = BlockState::Blue;
                    blue_pos = Some(red_pos);
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
                    for test_direction in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
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
                        } else if target_type == BlockState::Empty {
                            queue.push_back(move_target);
                            direction.insert(move_target, ideal_dir);
                        }
                    }
                }
                // move
                level_config.map[state.pos.1][state.pos.0] = BlockState::Empty;
                state.pos.0 = (state.pos.0 as i32 + ideal_direction.0) as usize;
                state.pos.1 = (state.pos.1 as i32 + ideal_direction.1) as usize;
                level_config.map[state.pos.1][state.pos.0] = BlockState::Monster;
                // check lose
                if red_pos == state.pos || blue_pos == Some(state.pos) {
                    level_ended = Some(num);
                }
            }

            Self::update_map(&resource, &map, &level_config.map);
            ctx.redraw();
            return true;
        });
    }
}
