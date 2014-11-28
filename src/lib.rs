use std::io::{MemWriter};

pub fn sha1(input: &[u8]) -> [u32, ..5] {
    //FIXME change this
    //let empty_output: [u8, ..20] = [
    //0xDA, 0x39, 0xA3, 0xEE, 0x5E, 0x6B, 0x4B, 0x0D, 0x32, 0x55, 0xBF, 0xEF, 0x95, 0x60, 0x18, 0x90, 0xAF, 0xD8, 0x07, 0x09];


    let mut h: [u32, ..5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

    let length: u32 = input.len() as u32;
    println!("Input lenght is {}", length);

    let mut cur_length: u32 = 0;
    for chunk in input.as_slice().chunks(64) {
        if chunk.len() == 64 {
            process_block(h.as_mut_slice(), chunk);
            cur_length += 64;
        }
        else {
            println!("handling last block and finishing up the thing");
            last_block(h.as_mut_slice(), input, cur_length);
        }
    }

    return h;
}

fn process_block(h: &mut [u32], block: &[u8]) {
    //println!("hex block: {}", hex(block));

        assert_eq!(block.len(), 64);

        let mut words = [0u32, ..80];
        for (i, chunk) in block.chunks(4).enumerate()
        {
            words[i] =
                    (chunk[3] as u32)
                |   (chunk[2] as u32 << 8)
                |   (chunk[1] as u32 << 16)
                |   (chunk[0] as u32 << 24)
            ;
        }

        let ff = |b: u32, c: u32, d: u32| d ^ (b & (c ^ d));
        let gg = |b: u32, c: u32, d: u32| b ^ c ^ d;
        let hh = |b: u32, c: u32, d: u32| (b & c) | (d & (b | c));
        let ii = |b: u32, c: u32, d: u32| b ^ c ^ d;

        let left_rotate = |x: u32, n: u32| (x << n as uint) | (x >> (32 - n) as uint);

        for i in range(16u, 80u)
        {
            let n = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            words[i] = left_rotate(n, 1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];

        for i in range(0u, 80u)
        {
            let (f, k) = match i {
                0...19  => (ff(b, c, d), 0x5a827999),
                20...39 => (gg(b, c, d), 0x6ed9eba1),
                40...59 => (hh(b, c, d), 0x8f1bbcdc),
                60...79 => (ii(b, c, d), 0xca62c1d6),
                _ => (0, 0),
            };

            let tmp = left_rotate(a, 5) + f + e + k + words[i];
            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = tmp;
        }

        h[0] += a;
        h[1] += b;
        h[2] += c;
        h[3] += d;
        h[4] += e;

}

fn last_block(h: &mut [u32], input: &[u8], cur_length: u32) {
    let length: u32 = input.len() as u32;

// m = SHA1-object. h = h.

    let mut w = MemWriter::new();
    w.write(input.as_slice());
    w.write_u8(0x80 as u8);
    w.write(Vec::from_elem((56u - (cur_length as uint)- 1u), 0x00 as u8).as_slice());
    w.write_be_u64((length * 8) as u64);

    process_block(h.as_mut_slice(), w.get_ref());

}

//fn bitrotate_l(block: &[u32], n: &[u8]) -> [u32] {
//    // bit-rotate block by n places
//}
//fn F(B: &[u32], C: &[u32], D: &[u32]) -> [u32] {
//    // the non-linear function that varies
//}


pub fn print_hex(buf: &[u32]) {
    //let out = "";
    for chunk in buf.iter() {
        print!("{:02x}",  *chunk        & 0xFF);
        print!("{:02x}", (*chunk >>  8) & 0xFF);
        print!("{:02x}", (*chunk >> 16) & 0xFF);
        print!("{:02x}", (*chunk >> 24) & 0xFF);
    }
    println!("");
}
