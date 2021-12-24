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

