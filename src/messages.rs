pub enum Message {
    SUCCESS(String),
    ERROR(String),
    VOID,
}

pub enum Signal {
    TRUE(u32),
    FALSE(u32),
}
