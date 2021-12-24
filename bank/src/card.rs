pub struct Card<'a> {
  pin: &'a [u8; 4],
  accounts: Vec<String>,
}

impl<'a> Card<'a> {
  pub fn new(pin: &'a [u8; 4], accounts: Vec<String>) -> Card<'a> {
    Card {
      pin: pin,
      accounts: accounts,
    }
  }

  pub fn verify(&self, pin: &'a [u8; 4]) -> bool {
    &*self.pin == &*pin
  }

  pub fn account_list(&self) -> Vec<String> {
    self.accounts.clone()
  }
}
