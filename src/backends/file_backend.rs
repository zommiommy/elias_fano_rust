use crate::codes::{CodeFixedLength, CodeUnary};
use crate::constants::*;
use crate::traits::*;
use crate::utils::{fast_log2_ceil, power_of_two_to_mask};
use crate::{Error, Result};
use core::mem::size_of;

use std::fs::File;
use std::io::BufReader;
use std::io::Read as IoRead;
use std::io::Seek as IoSeek;
use std::io::SeekFrom;
use std::io::Write as IoWrite;
use std::path::Path;

pub struct FileReader {
    buffer: [u8; size_of::<usize>()],
    file: BufReader<File>,
}

impl FileReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<FileReader> {
        let f = File::open(path).map_err(|e| Error::UnableToOpenFile(e.to_string()))?;

        Ok(FileReader {
            buffer: [0; size_of::<usize>()],
            file: BufReader::new(f),
        })
    }
}

/// TODO!: proper error handling and implement skip and rewind
impl Read for FileReader {
    fn read(&mut self) -> Result<usize> {
        self.file
            .read(&mut self.buffer)
            .map_err(|e| Error::ReadFailed(e.to_string()))?;

        Ok(usize::from_be_bytes(self.buffer))
    }

    fn seek(&mut self, word_offset: usize) -> Result<()> {
        self.file
            .seek(SeekFrom::Start((word_offset << WORD_SHIFT) as _))
            .unwrap();
        Ok(())
    }

    fn tell(&self) -> Result<usize> {
        Ok(self.file.stream_position().unwrap() as usize)
    }
}
