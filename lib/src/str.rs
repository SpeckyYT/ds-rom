use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct AsciiArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Display for AsciiArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.0 {
            write!(f, "{}", ch as char)?;
        }
        Ok(())
    }
}

pub(crate) fn write_blob_size(f: &mut std::fmt::Formatter<'_>, size: u32) -> std::fmt::Result {
    match size {
        0..=0x3ff => write!(f, "{} bytes", size),
        0x400..=0xfffff => write!(f, "{:.1}kB", size as f32 / 0x400 as f32),
        0x100000.. => write!(f, "{:.1}MB", size as f32 / 0x100000 as f32),
    }
}

pub(crate) fn write_hex(f: &mut std::fmt::Formatter<'_>, data: &[u8]) -> std::fmt::Result {
    for (offset, chunk) in data.chunks(16).enumerate() {
        write!(f, "{:08x} ", offset * 16)?;
        for byte in chunk {
            write!(f, " {byte:02x}")?;
        }
        writeln!(f)?;
    }
    writeln!(f)?;
    Ok(())
}

pub(crate) fn print_hex(data: &[u8]) {
    for (offset, chunk) in data.chunks(16).enumerate() {
        print!("{:08x} ", offset * 16);
        for byte in chunk {
            print!(" {byte:02x}");
        }
        println!();
    }
    println!();
}
