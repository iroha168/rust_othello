extern crate rand;
use rand::{Rng, thread_rng};
use std::{thread, time, mem};
use std::collections::{HashMap};
const BLACK: i32 = 1;
const WHITE: i32 = 2;
const MAX_EMPTY_FOR_FULL_SEARCH: usize = 24;
static BLACK_SYMBOL: &'static str = " X ";
static WHITE_SYMBOL: &'static str = " 0 ";
static EMPTY_SYMBOL: &'static str = " . ";
static AVAILABLE_SYMBOL: &'static str = " ? ";

fn available_moves(player: u64, opponent: u64) -> u64{
	let empties = !(player | opponent);
	let mut transitional_board: u64;
	let mut availables = 0;
	//  0x7e7e7e7e7e7e7e7e represents a bit baord filled
	//  with 1 in B to G columns and with 0 in A and H, like below.
	//
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  01111110

	let horizontal_opponent_mask = opponent & 0x7e7e7e7e7e7e7e7e;

	//leftward
	transitional_board = horizontal_opponent_mask & (player << 1);
	for _ in 0..5 {
		transitional_board |= horizontal_opponent_mask & (transitional_board << 1);
	}

	availables |= empties & (transitional_board << 1);

	//rightward
	transitional_board = horizontal_opponent_mask & (player >> 1);
	for _ in 0..5 {
		transitional_board |= horizontal_opponent_mask & (transitional_board >> 1);
	}
	availables |= empties & (transitional_board >> 1);

	//  0x00ffffffffffff00 represents a bit baord filled
	//  with 1 in 2 to 7 columns and with 0 in 0 and 8, like below.
	//
	//  00000000
	//  11111111
	//  11111111
	//  11111111
	//  11111111
	//  11111111
	//  11111111
	//  00000000
	//
	let vertical_opponent_mask = opponent & 0x00ffffffffffff00;

	//upward
	transitional_board = vertical_opponent_mask & (player << 8);
	for _ in 0..5 {
		transitional_board |= vertical_opponent_mask & (transitional_board << 8);
	}
	availables |= empties & (transitional_board << 8);

	//downward
	transitional_board = vertical_opponent_mask & (player >> 8);
	for _ in 0..5 {
		transitional_board |= vertical_opponent_mask & (transitional_board >> 8);
	}
	availables |= empties & (transitional_board >> 8);

	//  0x007e7e7e7e7e7e00 represents a bit baord filled
	//  with 1 in the inside and with 0 on the outside, like below.
	//
	//  00000000
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  01111110
	//  00000000
	//
	let diagonal_opponent_mask = opponent & 0x007e7e7e7e7e7e00;

	//upward to the right
	transitional_board = diagonal_opponent_mask & (player << 7);
	for _ in 0..5 {
		transitional_board |= diagonal_opponent_mask & (transitional_board << 7);
	}
	availables |= empties & (transitional_board << 7);

	//upward to the left
	transitional_board = diagonal_opponent_mask & (player >> 9);
	for _ in 0..5 {
		transitional_board |= diagonal_opponent_mask & (transitional_board >> 9);
	}
	availables |= empties & (transitional_board >> 9);

	//downward to the right
	transitional_board = diagonal_opponent_mask & (player << 9);
	for _ in 0..5 {
		transitional_board |= diagonal_opponent_mask & (transitional_board << 9);
	}
	availables |= empties & (transitional_board << 9);

	//downward to the left
	transitional_board = diagonal_opponent_mask & (player >> 7);
	for _ in 0..5 {
		transitional_board |= diagonal_opponent_mask & (transitional_board >> 7);
	}
	availables |= empties & (transitional_board >> 7);
	return availables;
}

fn reversed_stones(selected_move: u64, player: &u64, opponent: &u64) -> u64{
	let mut reversed: u64 = 0;
	for i in 0..=8{
		let mut unconfirmed_reversed: u64 = 0;
		let mut candidate = shift(selected_move, i);
		while(candidate != 0) && ((candidate & *opponent) != 0){
			unconfirmed_reversed|=candidate ;
			candidate = shift(candidate, i);
		}
		if (candidate & *player) != 0{
			reversed |= unconfirmed_reversed;
		}
	}
	return reversed;
}

fn shift(selected_move: u64, direction: i32) -> u64{
	match direction {
		0 => (selected_move << 8) & 0xffffffffffffff00,
		1 => (selected_move << 7) & 0x7f7f7f7f7f7f7f00,
		2 => (selected_move >> 1) & 0x7f7f7f7f7f7f7f7f,
		3 => (selected_move >> 9) & 0x007f7f7f7f7f7f7f,
		4 => (selected_move >> 8) & 0x00ffffffffffffff,
		5 => (selected_move >> 7) & 0x00fefefefefefefe,
		6 => (selected_move << 1) & 0xfefefefefefefefe,
		7 => (selected_move << 9) & 0xfefefefefefefe00,
		_ => 0
	}
}

fn make_move(selected_move: u64, player: &mut u64, opponent: &mut u64){
	let reversed = reversed_stones(selected_move, player, opponent);
	*player ^= selected_move | reversed;
	*opponent ^= reversed;
}

fn init_board()->(u64,u64){
	let black = (1 << 28) | (1 << 35);
	let white = (1 << 27) | (1 << 36);
	(black, white)
}

fn show_board(player: u64, opponent: u64, availables: u64, turn: i32){
    let player_symbol = if turn == BLACK {BLACK_SYMBOL} else {WHITE_SYMBOL};
    let opponent_symbol = if turn == BLACK {WHITE_SYMBOL} else {BLACK_SYMBOL};
	for i in 0..8{
		let player_on_ith_line = player >> (56 - i * 8);
		let opponent_on_ith_line = opponent >> (56 - i * 8);
		let available_on_ith_line = availables >> (56 - i * 8);
		let mut ith_line = String::new();
		for j in (0..8).rev(){
			if ((player_on_ith_line >> j) & 1) == 1{
				print!("{}",player_symbol);
			}
			else if ((opponent_on_ith_line >> j) & 1) == 1{
				print!("{}",opponent_symbol);
            }
			else if ((available_on_ith_line >> j) & 1) == 1{
				print!("{}",AVAILABLE_SYMBOL);
			}
            else{
				print!("{}",EMPTY_SYMBOL);
			}
		}
		println!("");
	}
	println!("");
}
fn count_stones(stones: u64) -> i32{
    let mut stone_num = stones;
    stone_num = (stone_num & 0x5555555555555555) + ((stone_num & 0xAAAAAAAAAAAAAAAA) >> 1);
    stone_num = (stone_num & 0x3333333333333333) + ((stone_num & 0xCCCCCCCCCCCCCCCC) >> 2);
    stone_num = (stone_num & 0x0F0F0F0F0F0F0F0F) + ((stone_num & 0xF0F0F0F0F0F0F0F0) >> 4);
    stone_num = (stone_num & 0x00FF00FF00FF00FF) + ((stone_num & 0xFF00FF00FF00FF00) >> 8);
    stone_num = (stone_num & 0x0000FFFF0000FFFF) + ((stone_num & 0xFFFF0000FFFF0000) >> 16);
    stone_num = (stone_num & 0x00000000FFFFFFFF) + ((stone_num & 0xFFFFFFFF00000000) >> 32);
    return stone_num as i32;
}

fn convert_num_to_move(pos_as_int: u32) -> String{
    let num_to_alpha: HashMap<u32, &str> = [(7,"a"),(6,"b"),(5,"c"),(4,"d"),(3,"e"),(2,"f"),(1,"g"),(0,"h")].iter().cloned().collect();
    let first = num_to_alpha[&(pos_as_int % 8)];
    let second = (8 - pos_as_int / 8).to_string();
    return format!("{}{}",first, second);
}

fn main() {
	let init_board = init_board();
	let mut black = init_board.0;
	let mut white = init_board.1;
	let mut turn = BLACK;
    let mut rng = thread_rng();
    let wait_time= time::Duration::from_millis(500);
    let mut player = black;
    let mut opponent = white;
    let availables = available_moves(player,opponent);
    show_board(player,opponent, availables, turn);
    let mut pass_cnt = 0;
    let mut move_cnt = 0;
    let mut is_target = true;
    let mut move_list = Vec::new();
    loop{
        let availables = available_moves(player,opponent);
        let player_display = if turn == BLACK {"Black"} else {"White"};
        if pass_cnt == 2{
            break;
        }
        show_board(player,opponent, availables, turn);
        if availables == 0{
            turn = 3 - turn;
            mem::swap(&mut player, &mut opponent);
            pass_cnt += 1;
            continue;
        }
        pass_cnt = 0;
        let mut selected_move:u64;
        loop{
            let x: u32 = rng.gen_range(0,64);
            selected_move = 2u64.pow(x);
            if selected_move & availables != 0{
            move_list.push(x);
                break;
            }
        }
        make_move(selected_move, &mut player, &mut opponent);
        move_cnt += 1;
        thread::sleep(wait_time);
        turn = 3 - turn;
        mem::swap(&mut player, &mut opponent);
    }
    println!("done!");
}
