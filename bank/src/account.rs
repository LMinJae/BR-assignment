pub struct Account<T> {
  balance: T,
}

impl<T: std::marker::Copy + std::cmp::PartialOrd + std::ops::AddAssign + std::ops::SubAssign> Account<T> {
  pub fn new(balance: T) -> Account<T> {
    Account {
      balance: balance,
    }
  }

  pub fn balance(&self) -> T {
    self.balance
  }

  pub fn deposit(&mut self, amount: T) -> Result<T, String> {
    self.balance += amount;
    Ok(self.balance)
  }

  pub fn withdraw(&mut self, amount: T) -> Result<T, String> {
    if self.balance < amount {
      return Err("Cannot withdraw more than own balance".to_string())
    }
    self.balance -= amount;
    Ok(self.balance)
  }
}
