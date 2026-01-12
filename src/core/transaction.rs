#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
}

impl Transaction {
    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }
}