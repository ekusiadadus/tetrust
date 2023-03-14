mod block;
mod game;
use block::{BlockKind, BLOCKS};
use game::{Field, Game, Position, FIELD_HEIGHT, FIELD_WIDTH};
use getch_rs::{Getch, Key};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn draw(Game { field, pos, block }: &Game) {
    let mut field_buf = field.clone();

    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }

    print!("{}[2J", 27 as char);
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                return true;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);

        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(100));
            let mut game = game.lock().unwrap();

            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game.field, &new_pos, game.block) {
                game.pos = new_pos;
            } else {
                let gy = game.pos.y;
                let gx = game.pos.x;
                for y in 0..4 {
                    for x in 0..4 {
                        if BLOCKS[game.block as usize][y][x] == 1 {
                            game.field[y + gy][x + gx] = 1;
                        }
                    }
                }
                for y in 0..FIELD_HEIGHT - 1 {
                    let mut can_erase = true;
                    for x in 0..FIELD_WIDTH - 1 {
                        if game.field[y][x] == 0 {
                            can_erase = false;
                            break;
                        }
                    }
                    if can_erase {
                        for y2 in (2..=y).rev() {
                            game.field[y2] = game.field[y2 - 1];
                        }
                    }
                }
                game.pos = Position::init();
                game.block = rand::random();
            }

            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h]");
                return;
            }
            _ => (),
        }
    }
}
