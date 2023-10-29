use itertools;
use itertools::repeat_n;
use std::mem;

pub struct Runtime {
    cells: Vec<u8>,
    ptr: usize,
    code: String,
    code_ptr: usize,
    jumps: Vec<Option<usize>>,
    jumps_map: Vec<usize>,
}

impl Runtime {
    const HEAP_LEN: usize = 200;

    pub fn new() -> Self {
        Self {
            cells: Vec::from_iter(itertools::repeat_n(0, Self::HEAP_LEN)),
            ptr: 0,
            code: String::new(),
            jumps: Vec::new(),
            jumps_map: Vec::new(),
            code_ptr: 0,
        }
    }

    pub fn with_code(code: String) -> Self {
        Self {
            cells: Vec::from_iter(itertools::repeat_n(0, Self::HEAP_LEN)),
            ptr: 0,
            code,
            jumps: Vec::new(),
            jumps_map: Vec::new(),
            code_ptr: 0,
        }
    }

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn next_output(&mut self) -> Option<u8> {
        loop {
            match self.code.chars().nth(self.code_ptr)? {
                '>' => {
                    self.ptr += 1;
                    if self.ptr >= Self::HEAP_LEN {
                        self.ptr -= Self::HEAP_LEN;
                    }
                }
                '<' => {
                    if self.ptr == 0 {
                        self.ptr = Self::HEAP_LEN - 1;
                    } else {
                        self.ptr -= 1;
                    }
                }
                '+' => {
                    self.cells[self.ptr] += 1;
                }
                '-' => {
                    self.cells[self.ptr] -= 1;
                }
                '.' => {
                    self.code_ptr += 1;
                    return Some(self.cells[self.ptr]);
                }
                '[' => {
                    if self.cells[self.ptr] == 0 {
                        self.code_ptr = self.jumps.get(self.code_ptr).copied()??;
                    }
                }
                ']' => {
                    if self.cells[self.ptr] != 0 {
                        self.code_ptr = self.jumps.get(self.code_ptr).copied()??;
                    }
                }
                '#' => {}
                _ => {}
            };
            self.code_ptr += 1;
        }
    }

    #[inline]
    pub fn set_code(&mut self, code: &str) -> Result<(), ()> {
        self.code.clear();
        self.code.push_str(code);
        self.reset_execution();
        self.recalculate_jumps(0, true)
    }

    #[inline]
    pub fn remove_append_code(&mut self, remove: usize, append: &str) -> Result<(), ()> {
        let prev = self.code.len();
        self.code.push_str(append);
        if append.contains(['[', ']']) {
            return self.recalculate_jumps(prev, false);
        }
        Ok(())
    }

    #[inline]
    pub fn append_code(&mut self, code: &str) -> Result<(), ()> {
        let prev = self.code.len();
        self.code.push_str(code);
        if code.contains(['[', ']']) {
            return self.recalculate_jumps(prev, false);
        }
        Ok(())
    }

    #[inline]
    fn recalculate_jumps(&mut self, start: usize, remove_old: bool) -> Result<(), ()> {
        if remove_old {
            self.jumps.iter_mut().for_each(|jump| *jump = None);
        }
        if self.code.len() > self.jumps.len() {
            self.jumps
                .extend(repeat_n(None, self.code.len() - self.jumps.len()));
        }

        self.jumps_map.clear();
        for (index, instruction) in self.code[start..].chars().enumerate() {
            match instruction {
                '[' => self.jumps_map.push(index),
                ']' => {
                    let other = self.jumps_map.pop().ok_or(())?;
                    unsafe {
                        *self.jumps.get_unchecked_mut(other) = Some(index);
                        *self.jumps.get_unchecked_mut(index) = Some(other)
                    };
                }
                _ => {}
            }
        }
        self.jumps_map.clear();
        Ok(())
    }

    #[inline]
    pub fn reset_execution(&mut self) {
        self.cells.iter_mut().for_each(|bit| *bit = 0);
        self.ptr = 0;
        self.code_ptr = 0;
    }
}
