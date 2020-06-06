use std::cmp::Ordering;

pub fn create_table() -> Vec<i16>{
	//returneaza starea initiala a jocului
	return vec![2, 0, 0, 0, 0, -5,
	 0, -3, 0, 0, 0, 5,
	 -5, 0, 0, 0, 3, 0,
	 5, 0, 0, 0, 0, -2];
}

fn print_indexes(table: &Vec<i16>){//fara & ramane ownership-ul vec aici
	for i in (6..12).rev(){
		print!("{}  ", i);
	}	

	print!("| ");

	for i in (0..6).rev(){
		print!("{}  ", i);
	}

	print!("\n");

	for i in 12..table.len(){
		if i == 18{
			print!("  | ");
		}

		print!("{} ", i);
	}

	println!("\n");
}

pub fn print_table(table: &Vec<i16>){
	println!();

	for i in (6..12).rev(){
		match table[i].cmp(&0){
	    	Ordering::Less => print!("{}{} ", 'n', table[i].abs()),//piesa neagra
	    	Ordering::Greater => print!("{}{} ", 'a', table[i].abs()),//piesa alba
	    	Ordering::Equal => print!("{} ", "##"),//spatiu gol
		}
	}	

	print!(" | ");

	for i in (0..6).rev(){
		match table[i].cmp(&0){
	    	Ordering::Less => print!("{}{} ", 'n', table[i].abs()),//piesa neagra
	    	Ordering::Greater => print!("{}{} ", 'a', table[i].abs()),//piesa alba
	    	Ordering::Equal => print!("{} ", "##"),//spatiu gol
		}	
	}

	print!("\n");

	for i in 12..table.len(){
		if i == 18{
			print!(" | ");
		}

		match table[i].cmp(&0){
	    	Ordering::Less => print!("{}{} ", 'n', table[i].abs()),//piesa neagra
	    	Ordering::Greater => print!("{}{} ", 'a', table[i].abs()),//piesa alba
	    	Ordering::Equal => print!("{} ", "##"),//spatiu gol
		}
	}

	println!("\n\nxn, x - piesa a(lba) sau n(eagra) si n - count piese\n");

	print_indexes(table);
}

fn get_multiply_player(player: &char) -> i16{
	let mut mutiply_player = 1;//daca player == 'a'
	
	if player == &'n'{
		mutiply_player = -1;//daca pleyer == 'n'
	}

	return mutiply_player;
}

pub fn validate_choice(poz: usize, table: &Vec<i16>, player: &char) -> bool{
	//validare selectare piesa pentru jucatorul player
	if poz > table.len(){
		println!("Inputul trebuie sa fie unul din indicii de pe tabla!");
		return false;
	}

	if table[poz] == 0{
		println!("Pozitie goala.");
		return false;
	}

	let mutiply_player = get_multiply_player(&player);

	if table[poz] * mutiply_player < 0{
		println!("Pozitie invalida pentru jucatorul {}.", player);
		return false;
	}

	return true;
}

pub fn validate_move(poz: usize, table: &Vec<i16>, player: &char, piece: usize) -> bool{
	//validare mutare 
	if poz >= table.len(){
		println!("Inputul trebuie sa fie unul din indicii de pe tabla!");
		return false;
	}

	if player == &'n' && poz >= piece{
		println!("Piesele nu pot merge inapoi.");
		return false;
	}

	if player == &'a' && poz <= piece{
		println!("Piesele nu pot merge inapoi.");
		return false;
	}

	let mutiply_player = get_multiply_player(&player);

	//daca pe pozitia dorita inamicul are poarta
	if table[poz] * mutiply_player < -1{
		println!("Inamicul are poarta pe pozitia {}.", poz);
		return false;
	}

	return true;
}

pub fn make_move(piece: usize, to_move: usize, table: Vec<i16>, player: &char) -> Vec<i16>{
	let mut aux_table = table; 

	let mutiply_player = get_multiply_player(&player);
	aux_table[piece] += -1 * mutiply_player;
	
	if aux_table[to_move] * mutiply_player < 0{//daca s-a mancat o piesa
		//flag
		//de implementat ceva sa tina minta minte cate piese sunt mancate si sa blocheze playerul

		aux_table[to_move] = 0;
	}

	aux_table[to_move] += 1 * mutiply_player;
	//s-a mancat o piesa

	return aux_table;
}

pub fn check_final(table: &Vec<i16>, player: char) -> char{
	//returneaza simbolul castigator sau x daca inca nu e gata
	let mutiply_player = get_multiply_player(&player);
	let mut counter = 0;

	for i in 0..table.len(){
		if table[i] * mutiply_player > 0{
			counter += 1;
		}
	}

	if counter == 0{
		println!("\nA castigat {}!", player);
		return player;
	}

	return 'x';
}