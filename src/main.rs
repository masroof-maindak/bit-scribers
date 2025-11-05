use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
enum VarLenInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Hex(String),
}

fn parse_args(args: &[String]) -> Result<(&String, &String), String> {
    if args.len() != 3 {
        let msg = format!(
            "Invalid argument count.\nUsage: {} <fpath> <byte offset>",
            args[0]
        );

        return Err(msg);
    }

    let fpath = &args[1];
    let offset_str = &args[2];

    Ok((fpath, offset_str))
}

fn vec_to_int(buf: Vec<u8>) -> VarLenInt {
    let mut ret: VarLenInt;
    let byte_count = buf.len();

    if byte_count > 16 {
        // FIXME: combine into single string
        let s = format!("0x{buf:x?}");
        ret = VarLenInt::Hex(s);
        return ret;
    }

    if byte_count <= 1 {
        ret = VarLenInt::U8(0)
    } else if byte_count <= 2 {
        ret = VarLenInt::U16(0)
    } else if byte_count <= 4 {
        ret = VarLenInt::U32(0)
    } else if byte_count <= 8 {
        ret = VarLenInt::U64(0)
    } else {
        ret = VarLenInt::U128(0)
    }

    let mut bits_to_shift = 8 * (byte_count);

    for byte in buf {
        bits_to_shift -= 8;

        // Legacy from the union era:
        //
        // unsafe { ret.U128 += u128::from(byte) << shift_idx };

        ret = match ret {
            VarLenInt::U8(v) => VarLenInt::U8((v + byte) << bits_to_shift),
            VarLenInt::U16(v) => VarLenInt::U16((v + u16::from(byte)) << bits_to_shift),
            VarLenInt::U32(v) => VarLenInt::U32((v + u32::from(byte)) << bits_to_shift),
            VarLenInt::U64(v) => VarLenInt::U64((v + u64::from(byte)) << bits_to_shift),
            VarLenInt::U128(v) => VarLenInt::U128((v + u128::from(byte)) << bits_to_shift),
            VarLenInt::Hex(v) => {
                unreachable!("The hex string's ({v}) case should have been catered to above...")
            }
        }
    }

    ret
}

fn scan_from_file(mut f: File, offset: u64) -> io::Result<Vec<u8>> {
    let mut readbuf: Vec<u8> = vec![0; 8];
    let mut buf: Vec<u8> = Vec::new();
    let mut n_bytes: usize = 0;

    // CHECK/TODO: catch out-of-bounds file read when starting

    loop {
        f.seek(io::SeekFrom::Start(offset + n_bytes as u64))?;

        let bytes_read = f.read(&mut readbuf)?;
        let mut local_n_bytes = 0;

        for byte in &readbuf {
            if *byte >= 128 {
                buf.push(*byte);
                n_bytes += 1;
                local_n_bytes += 1;
            } else {
                break;
            }
        }

        // if the latest byte was greater than 128, and we've
        // exhausted the 'read buffer' this iteration

        if let Some(last) = buf.last()
            && *last >= 128
            && local_n_bytes == bytes_read
        {
            continue;
        }

        buf.push(readbuf[local_n_bytes]);
        break;
    }

    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (fpath, offset_str) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let offset = offset_str
        .parse::<u64>()
        .expect("Offset should be an integer.");

    let f = File::open(fpath)?;

    let buf = scan_from_file(f, offset)?;

    let x = vec_to_int(buf);

    println!("{x:?}");

    Ok(())
}
