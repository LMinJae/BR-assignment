pub static BANK: state::Storage<std::sync::RwLock<bank::Dummy>> = state::Storage::new();

pub fn init() -> bool{
	BANK.set(std::sync::RwLock::new(bank::Dummy::new()))
}

pub struct Session {
  uuid: uuid::Uuid,
}

impl Session {
  pub fn new() -> Session {
    use bank::Bank;

    let mut b = crate::BANK.get().write().unwrap();

    Session {
      uuid: b.create_session(),
    }
  }
}

pub mod card {
  #[allow(dead_code)]
  pub fn insert_card(s: &crate::Session, card_number: String) -> Result<(), String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.insert_card(&s.uuid, card_number)
  }

  #[allow(dead_code)]
  pub fn verify_pin<'a>(s: &crate::Session, pin: &'a [u8; 4]) -> Result<bool, String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.verify_pin(&s.uuid, pin)
  }

  #[allow(dead_code)]
  pub fn account_list(s: &crate::Session) -> Result<Vec<String>, String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.account_list(&s.uuid)
  }

  #[allow(dead_code)]
  pub fn account_select<'a>(s: &crate::Session, account_number: String) -> Result<(), String>{
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.account_select(&s.uuid, account_number)
  }
}

pub mod account {
  #[allow(dead_code)]
  pub fn balance(s: &crate::Session) -> Result<u64, String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.balance(&s.uuid)
  }

  #[allow(dead_code)]
  pub fn deposit(s: &crate::Session, amount: u64) -> Result<u64, String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.deposit(&s.uuid, amount)
  }

  #[allow(dead_code)]
  pub fn withdraw(s: &crate::Session, amount: u64) -> Result<u64, String> {
    use bank::Bank;
    let mut b = crate::BANK.get().write().unwrap();

    b.withdraw(&s.uuid, amount)
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
      crate::init();
      let s = crate::Session::new();
      
      assert_eq!(crate::card::insert_card(&s, "1234567887654321".to_string()), Ok(()));
      {
        let mut pin: [u8; 4] = Default::default();
        pin.copy_from_slice(b"3579");
        assert_eq!(crate::card::verify_pin(&s, &pin), Ok(true));
      }
      assert_eq!(crate::card::account_select(&s, "10010001000".to_string()), Ok(()));
      assert_eq!(crate::account::balance(&s), Ok(10000));
    }
}
