use std::fmt;

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const USERNAME_OFFSET: usize = ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;


const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;



#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}

#[derive(Debug)]
pub struct Table {
    pub num_rows: usize,
    pub pages: [*mut u8; 100],
}