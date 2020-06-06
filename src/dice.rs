use rand::Rng;

fn get_random_values() -> (usize, usize){
	//returneaza doua valori random intre 1 si 6
	let mut rng = rand::thread_rng();

	let n1 = (rng.gen::<usize>() % 6) + 1;
	let n2 = (rng.gen::<usize>() % 6) + 1;

	let pair = (n1, n2);
	return pair;
}

pub fn start_player() -> char{
	let pair1 = get_random_values();
	println!("Aruncam zaruri pentru albe: {:?}", pair1);
	
	let pair2 = get_random_values();
	println!("Aruncam zaruri pentru negre: {:?}", pair2);
	
	let sum1 = pair1.0 + pair1.1;
	let sum2 = pair2.0 + pair2.1;

	if sum1 > sum2{
		println!("Albele incep.");
		return 'a';
	}

	println!("Negrele incep.");
	return 'n';
}

fn evaluate_result(dice: &(usize, usize)) -> u8{
	print!("\nZar = {:?}. ", dice);

	if dice.0 == dice.1{
		println!("Ai 4 mutari.");
		return 4;
	}
	
	println!("Ai 2 mutari.");
	return 2;
}

pub fn roll_dice() -> (usize, usize, u8){
	//returneaza doua zaruri aruncate si numarul de mutari
	let pair = get_random_values();

	let moves = evaluate_result(&pair);

    return (pair.0, pair.1, moves); 
}