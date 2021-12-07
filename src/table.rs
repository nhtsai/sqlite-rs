use std::fmt;

/*
 * Row Constants
 */
const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const USERNAME_OFFSET: usize = ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

/*
 * Page Constants
 */
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    /// Serializes a Row to compact representation.
    fn serialize(row: &Row) -> Vec<u8> {
        let mut buf = vec![0; ROW_SIZE];
        buf
    }

    fn deserialize(buf: &Vec<u8>) -> Row {
        let mut row = Row {
            id: 1,
            username: String::from("user"),
            email: String::from("user@example.com"),
        };
        row
    }
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
