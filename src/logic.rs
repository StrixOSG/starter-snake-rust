use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game};

pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "strixos",
        "color": "#f5bf42",
        "head": "silly",
        "tail": "mlh-gene"
    });
}

pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    let my_head = &you.body[0];
    let my_neck = &you.body[1];
    
    if my_neck.x < my_head.x {
        is_move_safe.insert("left", false);

    } else if my_neck.x > my_head.x {
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y {
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y {
        is_move_safe.insert("up", false);
    }

    let board_width = &_board.width;
    let board_height = &_board.height;
    if my_head.x == board_width - 1 {
        is_move_safe.insert("right", false);
    } else if my_head.x == 0 {
        is_move_safe.insert("left", false);
    }

    if my_head.y == board_height - 1 {
        is_move_safe.insert("up", false);
    } else if my_head.y == 0 {
        is_move_safe.insert("down", false);
    }

    let my_body = &you.body;
    info!("TURN {}", turn);
    info!("HEAD {}: {}", my_head.x, my_head.y);
    for part in &my_body[1..] {
        info!("BODY {}: {}", part.x, part.y);
        if my_head.x == part.x && my_head.y + 1 == part.y {
            is_move_safe.insert("up", false);
            info!("NOT SAFE: up");
        }
        if my_head.x == part.x && my_head.y == part.y + 1 {
            is_move_safe.insert("down", false);
            info!("NOT SAFE: down");
        }
        if my_head.y == part.y && my_head.x + 1 == part.x {
            is_move_safe.insert("right", false);
            info!("NOT SAFE: right");
        }
        if my_head.y == part.y && my_head.x == part.x + 1 {
            is_move_safe.insert("left", false);
            info!("NOT SAFE: left");
        }
    }

    let opponents = &_board.snakes;
    info!("TURN {}", turn);
    info!("HEAD {}: {}", my_head.x, my_head.y);
    for opponent in opponents {
        if opponent.id == you.id {
            continue;
        }

        for part in &opponent.body {
            info!("PART {}: {}", part.x, part.y);
            if my_head.x == part.x && my_head.y == part.y + 1 {
                is_move_safe.insert("down", false);
                info!("NOT SAFE: down");
            }
            if my_head.x == part.x && my_head.y + 1 == part.y {
                is_move_safe.insert("up", false);
                info!("NOT SAFE: up");
            }
            if my_head.y == part.y && my_head.x == part.x + 1 {
                is_move_safe.insert("left", false);
                info!("NOT SAFE: left");
            }
            if my_head.y == part.y && my_head.x + 1 == part.x {
                is_move_safe.insert("right", false);
                info!("NOT SAFE: right");
            }
            if you.length <= opponent.length {
                if my_head.x == opponent.head.x && my_head.y == opponent.head.y + 1 || my_head.y ==  opponent.head.y + 2 {
                    is_move_safe.insert("down", false);
                    info!("NOT SAFE: down");
                }
                if my_head.x == opponent.head.x && my_head.y + 1 == opponent.head.y || my_head.y + 2 == opponent.head.y {
                    is_move_safe.insert("up", false);
                    info!("NOT SAFE: up");
                }
                if my_head.y == opponent.head.y && my_head.x == opponent.head.x + 1 || my_head.x == opponent.head.x + 2 {
                    is_move_safe.insert("left", false);
                    info!("NOT SAFE: left");
                }
                if my_head.y == opponent.head.y && my_head.x + 1 == opponent.head.x || my_head.x + 2 == opponent.head.x {
                    is_move_safe.insert("right", false);
                    info!("NOT SAFE: right");
                }
            }
        }
    }

    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    let mut chosen = "left";

    if !safe_moves.is_empty() {
        chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();
    }

    if you.length < 7 {
        let food = &_board.food;
        let mut nearest_food = None;
        let mut min_distance = std::i32::MAX;
        for f in food {
            let distance = (my_head.x as i32 - f.x as i32).abs() + (my_head.y as i32 - f.y as i32).abs();
            if distance < min_distance {
                min_distance = distance;
                nearest_food = Some(f);
            }
        }
        if nearest_food.is_some() {
            let close_food = nearest_food.unwrap();
            let dir_x = close_food.x as i32 - my_head.x as i32;
            let dir_y = close_food.y as i32 - my_head.y as i32;
                if dir_x < 0 && safe_moves.contains(&"left") {
                    chosen = &"left";
                    info!("FOOD: left");
                } else if dir_x > 0 && safe_moves.contains(&"right") {
                    chosen = &"right";
                    info!("FOOD: right");
                } else if dir_y > 0 && safe_moves.contains(&"up") {
                    chosen = &"up";
                    info!("FOOD: up");
                } else if dir_y < 0 && safe_moves.contains(&"down") {
                    chosen = &"down";
                    info!("FOOD: down");
                }
        }
    }

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
