// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game, Coord};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
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

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    
    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
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

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;
    for part in &my_body[1..] {
        if my_head.x == part.x && my_head.y + 1 == part.y {
            is_move_safe.insert("up", false);
        } else if my_head.x == part.x && my_head.y == part.y + 1 {
            is_move_safe.insert("down", false);
        } else if my_head.y == part.y && my_head.x + 1 == part.x {
            is_move_safe.insert("left", false);
        } else if my_head.y == part.y && my_head.x == part.x + 1 {
            is_move_safe.insert("right", false);
        }
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = &_board.snakes;
    for opponent in opponents {
        // Check each part of opponent snakes
        for part in &opponent.body {
            if my_head.x == part.x && my_head.y == part.y + 1 {
                is_move_safe.insert("up", false);
            } else if my_head.x == part.x && my_head.y + 1 == part.y {
                is_move_safe.insert("down", false);
            } else if my_head.y == part.y && my_head.x == part.x + 1 {
                is_move_safe.insert("left", false);
            } else if my_head.y == part.y && my_head.x + 1 == part.x {
                is_move_safe.insert("right", false);
            }
        }
    }

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    // Choose a random move from the safe ones
    let mut chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
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
        let dir_x = my_head.x as i32 - close_food.x as i32;
        let dir_y = my_head.y as i32 - close_food.y as i32;
            if dir_x < 0 && safe_moves.contains(&"left") {
                chosen = &"left";
            } else if dir_x > 0 && safe_moves.contains(&"right") {
                chosen = &"right";
            } else if dir_y > 0 && safe_moves.contains(&"up") {
                chosen = &"up";
            } else if dir_y < 0 && safe_moves.contains(&"down") {
                chosen = &"down";
            }
    }

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
