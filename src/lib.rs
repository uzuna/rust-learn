pub mod first;
pub mod second;

pub enum SortOrder {
    Ascending,
    Descending,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
