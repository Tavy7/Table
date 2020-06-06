use std::io;

mod game;
mod dice;

fn select_index(table: &Vec<i16>, current_player: &char) -> usize{
	let ok;
	
	game::print_table(&table);
	print!("Introduceti piesa de mutat: ");
	io::Write::flush(&mut io::stdout()).expect("flush failed!");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Eroare la citire!");

	let numar: usize = input.trim().parse().expect("Inputul trebuie sa fie un numar!");
		
	ok = game::validate_choice(numar, &table, &current_player);//verifica daca piesa e buna

	if ok == true{
		return numar;
	}

	return 30;
}


fn switch_player(current_player: char) -> char{

	if current_player == 'n'{
		return 'a';
	}

	return 'n';
}

fn main() {
	let mut table = game::create_table();

	println!("Decidem cine incepe.");
	let mut current_player = dice::start_player();//char = 'a' sau 'n'
	let mut dice;
	loop{
		dice = dice::roll_dice();
		
		for _i in 0..dice.2 / 2{//dice.2 e numarul de mutari, impartit la 2 deoarece in loop mutam pentru ambele zaruri
			
			let mut poz;
			let mut selected_piece;

			loop{
				println!("\nJucator curent: {}.", current_player);
				println!("Mutam cu valoarea: {}.", dice.0);

				selected_piece = select_index(&table, &current_player);
				if current_player == 'a'{
					if game::validate_move(selected_piece + dice.0, &table, &current_player, selected_piece){
						poz = selected_piece + dice.0;
						break;
					}
				}

				if current_player == 'n'{
					if game::validate_move(selected_piece - dice.0, &table, &current_player, selected_piece){
						poz = selected_piece - dice.0;
						break;
					}
				}
			}

			//efectueaza mutare
			table = game::make_move(selected_piece, poz, table, &current_player);

			loop{
				println!("\nJucator curent: {}.", current_player);
				println!("Mutam cu {}.", dice.1);

				selected_piece = select_index(&table, &current_player);

				if selected_piece == 30{
					continue;
				}

				if current_player == 'a'{
					if game::validate_move(selected_piece + dice.1, &table, &current_player, selected_piece){
						poz = selected_piece + dice.1;
						break;
					}
				}

				if current_player == 'n'{
					if game::validate_move(selected_piece - dice.1, &table, &current_player, selected_piece){
						poz = selected_piece - dice.1;
						break;
					}
				}
			}
			//efectueaza mutare
			table = game::make_move(selected_piece, poz, table, &current_player);
		}

		if game::check_final(&table, current_player) == current_player{
			break;
		}

		current_player = switch_player(current_player);
	}
}