use crate::Result;
// BytePacketBuffer with dynamic resizing
pub struct BytePacketBuffer {
    pub buf: Vec<u8>,
    pub pos: usize,
}

#[allow(dead_code)]
impl BytePacketBuffer {
    pub fn reset(&mut self) -> Result<()> {
        self.clear();
        Ok(())
    }
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: vec![0; 512], // Initial size, will be resized if needed
            pos: 0,
        }
    }
    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        self.ensure_capacity(self.pos);
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        self.ensure_capacity(self.pos);
        Ok(())
    }

    fn ensure_capacity(&mut self, capacity: usize) {
        if capacity > self.buf.len() {
            let new_capacity = std::cmp::max(capacity, self.buf.len() * 2);
            self.buf.resize(new_capacity, 0);
        }
    }

    pub fn read(&mut self) -> Result<u8> {
        if self.pos >= self.buf.len() {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }
    pub fn get(&self, pos: usize) -> Result<u8> {
        if pos >= self.buf.len() {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len > self.buf.len() {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut delim = "";
        let max_jumps = 5;
        let mut jumps_performed = 0;
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }
            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                jumps_performed += 1;
                continue;
            }
            pos += 1;
            if len == 0 {
                break;
            }
            outstr.push_str(delim);
            let str_buffer = self.get_range(pos, len as usize)?;
            outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
            delim = ".";
            pos += len as usize;
        }
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }

    pub fn write(&mut self, val: u8) -> Result<()> {
        self.ensure_capacity(self.pos + 1);
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        self.write(val)?;
        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;
        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;
        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<()> {
        for label in qname.split('.') {
            let len = label.len();
            if len > 0x34 {
                return Err("Single label exceeds 63 characters of length".into());
            }
            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }
        self.write_u8(0)?;
        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<()> {
        if pos >= self.buf.len() {
            return Err("Out of bounds".into());
        }
        self.buf[pos] = val;
        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<()> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.buf.resize(512, 0);
        self.pos = 0;
    }
}
