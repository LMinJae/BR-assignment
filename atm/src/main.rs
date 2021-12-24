fn read_line() -> String {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	line.trim().to_string()
}

fn main() -> std::io::Result<()> {
	bank_api::init();

	let s = bank_api::Session::new();

	// Insert Card
	{
		println!("Input card number");
		let card_number = read_line();
		bank_api::card::insert_card(&s, card_number).unwrap();
	}

	// verift PIN number
	{
		let mut pin: [u8; 4] = Default::default();
		{
			let mut line: String;
			loop {
				println!("Input PIN number");
				line = read_line();

				if 4 == line.len() {
					break;
				} else {
					println!("PIN number is only support 4 digit");
				}
			}
			pin.copy_from_slice(&line.as_bytes()[0..4]);
		}
		match bank_api::card::verify_pin(&s, &pin) {
			Err(e) => {
				println!("{}", e);
				return Ok(())
			},
			Ok(false) => {
				println!("Pin number mismatch");
				return Ok(())
			},
			Ok(true) => {
				match bank_api::card::account_list(&s) {
					Err(e) => {
						println!("{}", e);
					},
					Ok(lst) => {
						println!("List of accounts");

						for i in &lst {
							println!("{}", i);
						}
					},
				}
			}
		};
	}

	// Select Account
	{
		println!("Input account number");
		let account_number = read_line();
		match bank_api::card::account_select(&s, account_number) {
			Err(e) => {
				println!("{}", e);
				return Ok(())
			},
			Ok(()) => {
				// See Balance/Deposit/Withdraw

				loop {
					println!("[B]alance/[D]eposit/[W]ithdraw/[E]ixt");
					let cmd = read_line();
					match cmd.as_bytes() {
						b"B" => {
							match bank_api::account::balance(&s) {
								Err(e) => {
									println!("{}", e);
								},
								Ok(b) => {
									println!("Balance: {}", b);
								},
							}
						},
						b"D" => {
							let amount: u64 = read_line().parse().unwrap();

							match bank_api::account::deposit(&s, amount) {
								Err(e) => {
									println!("{}", e);
								},
								Ok(b) => {
									println!("Balance: {}", b);
								}
							}
						},
						b"W" => {
							let amount: u64 = read_line().parse().unwrap();

							match bank_api::account::withdraw(&s, amount) {
								Err(e) => {
									println!("{}", e);
								},
								Ok(b) => {
									println!("Balance: {}", b);
								}
							}
						},
						b"E" => {
							break
						},
						_ => {
							println!("Wrong command input");
						}
					}
				}
			},
		}
	}

	Ok(())
}
