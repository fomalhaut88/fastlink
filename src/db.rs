use std::{io, fs};
use std::os::unix::prelude::FileExt;
use std::io::prelude::*;


#[derive(Debug)]
pub struct DBConnector {
    prime: u64,
    generator: u64,
    state: u64,
    block_size: usize,
    data_file: fs::File,
    state_file: fs::File,
}


impl DBConnector {
    pub fn new(data_path: &str, state_path: &str, prime: u64,
               generator: u64, state: u64, block_size: usize) -> Self {
        assert!(generator < prime, "generator cannot be bigger than prime");
        assert!(generator != 0, "generator cannot be 0");
        assert!(generator != 1, "generator cannot be 1");
        assert!(state < prime, "state cannot be bigger than prime");

        let data_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_path).unwrap();

        let state_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(state_path).unwrap();

        Self {
            prime, generator,
            state: if state == 0 { generator } else { state },
            block_size, data_file, state_file
        }
    }

    pub fn get_state(&self) -> u64 {
        self.state
    }

    pub fn evolve_state(&mut self) -> u64 {
        self.state = ((
            self.state as u128 * self.generator as u128
        ) % self.prime as u128) as u64;
        self.state
    }

    pub fn datafile_is_empty(&self) -> bool {
        self.data_file.metadata().unwrap().len() == 0
    }

    pub fn statefile_is_empty(&self) -> bool {
        self.state_file.metadata().unwrap().len() == 0
    }

    pub fn check_datafile(&self) {
        let correct_data_size = (self.prime - 1) as usize * self.block_size;
        let actual_data_size = self.data_file.metadata().unwrap().len() as usize;
        assert!(correct_data_size == actual_data_size, "invalid size of datafile");
    }

    pub fn init_datafile(&mut self) -> Result<(), io::Error> {
        const CHUNK_SIZE: usize = 1048576;
        let buffer = [0u8; CHUNK_SIZE];
        let mut left_bytes = (self.prime - 1) as usize * self.block_size;
        while left_bytes > 0 {
            if left_bytes > CHUNK_SIZE {
                self.data_file.write_all(&buffer)?;
                left_bytes -= CHUNK_SIZE;
            } else {
                self.data_file.write_all(&buffer[..left_bytes])?;
                left_bytes = 0;
            }
        }
        Ok(())
    }

    pub fn load_state(&mut self) -> Result<(), io::Error> {
        let mut block = [0u8; 8];
        self.state_file.read_at(&mut block, 0)?;
        self.state = u64::from_ne_bytes(block);
        Ok(())
    }

    pub fn save_state(&self) -> Result<(), io::Error> {
        let block = self.state.to_ne_bytes();
        self.state_file.write_at(&block, 0)?;
        Ok(())
    }

    pub fn get(&self, idx: u64) -> Result<Vec<u8>, io::Error> {
        let mut block: Vec<u8> = vec![0; self.block_size];
        self.data_file.read_exact_at(&mut block, idx * self.block_size as u64)?;
        Ok(block)
    }

    pub fn set(&self, idx: u64, block: &[u8]) -> Result<(), io::Error> {
        self.data_file.write_all_at(block, idx * self.block_size as u64)?;
        Ok(())
    }

    pub fn ensure(&mut self, order: usize) {
        assert!(self.prime < (1 << (6 * order)), "too big prime");

        if self.datafile_is_empty() {
            self.init_datafile().unwrap();
        }
        self.check_datafile();

        if self.statefile_is_empty() {
            self.save_state().unwrap();
        }
        self.load_state().unwrap();
    }
}
