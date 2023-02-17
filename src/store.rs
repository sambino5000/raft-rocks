use rocksdb::{Error, Options, DB};

pub struct KeyValueStoreHandler {
    db: DB,
}
impl KeyValueStoreHandler {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.set_keep_log_file_num(1);
        opts.create_if_missing(true);
        let db = DB::open(&opts, &path).unwrap();
        KeyValueStoreHandler { db }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        match self.db.get(key) {
            Ok(Some(value)) => println!(
                "retrieved value {}",
                String::from_utf8(value.to_vec()).unwrap()
            ),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        };

        self.db.get(key)
    }

    pub fn put(path: &str, key: &[u8], value: &[u8]) -> Result<(), String> {
        let mut opts = Options::default();
        opts.set_keep_log_file_num(1);
        opts.create_if_missing(true);
        let db = DB::open(&opts, &path).unwrap();
        match db.put(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
