use ic_cdk_macros::{init, query, update};
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Account {
    balance: u64,
}

type AccountId = String;

#[derive(Default)]
struct Token {
    total_supply: u64,
    balances: HashMap<AccountId, Account>,
    owner: AccountId,
    symbol: String,
    name: String,
    decimals: u8,
}

impl Token {
    fn new(symbol: String, name: String, decimals: u8, total_supply: u64, owner: AccountId) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner.clone(), Account { balance: total_supply });
        
        Token {
            total_supply,
            balances,
            owner,
            symbol,
            name,
            decimals,
        }
    }

    fn balance_of(&self, account: &AccountId) -> u64 {
        self.balances.get(account).map_or(0, |acc| acc.balance)
    }

    fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: u64) -> Result<(), String> {
        let from_balance = self.balances.get_mut(from).ok_or("Sender account not found")?;
        
        if from_balance.balance < amount {
            return Err("Insufficient balance".to_string());
        }

        from_balance.balance -= amount;
        
        let to_balance = self.balances.entry(to.clone()).or_insert(Account { balance: 0 });
        to_balance.balance += amount;

        Ok(())
    }
}

static mut TOKEN: Option<Token> = None;

#[init]
fn init_token(owner: AccountId, total_supply: u64, symbol: String, name: String, decimals: u8) {
    let token = Token::new(symbol, name, decimals, total_supply, owner);
    unsafe {
        TOKEN = Some(token);
    }
}

#[query]
fn get_balance(account: AccountId) -> u64 {
    unsafe {
        if let Some(token) = &TOKEN {
            return token.balance_of(&account);
        }
        0
    }
}

#[update]
fn transfer(from: AccountId, to: AccountId, amount: u64) -> Result<(), String> {
    unsafe {
        if let Some(token) = &mut TOKEN {
            return token.transfer(&from, &to, amount);
        }
        Err("Token contract not initialized".to_string())
    }
}
