pub struct Sha1 {
    h0: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
}

impl Sha1 {
    pub fn new() -> Sha1 {
        Sha1 {
            h0: 0x67452301,
            h1: 0xEFCDAB89,
            h2: 0x98BADCFE,
            h3: 0x10325476,
            h4: 0xC3D2E1F0,
        }
    }

    fn _left_rotate(x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32 - n))
    }

    fn _process_block(&mut self, block: &[u8]) {
        let mut w: [u32; 80] = [0; 80];
        for i in 0..16 {
            w[i] = (block[i * 4] as u32) << 24 
                 | (block[i * 4 + 1] as u32) << 16 
                 | (block[i * 4 + 2] as u32) << 8 
                 | (block[i * 4 + 3] as u32);
        }
        for i in 16..80 {
            // SHA-1 标准：w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]) <<< 1
            w[i] = Sha1::_left_rotate(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }

        let mut a = self.h0;
        let mut b = self.h1;
        let mut c = self.h2;
        let mut d = self.h3;
        let mut e = self.h4;

        for i in 0..80 {
            // 修正：根据 i 的范围选择不同的 f 和 k
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            let temp = Sha1::_left_rotate(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i])
                .wrapping_add(k); // 使用正确的 k

            e = d;
            d = c;
            c = Sha1::_left_rotate(b, 30);
            b = a;
            a = temp;
        }

        self.h0 = self.h0.wrapping_add(a);
        self.h1 = self.h1.wrapping_add(b);
        self.h2 = self.h2.wrapping_add(c);
        self.h3 = self.h3.wrapping_add(d);
        self.h4 = self.h4.wrapping_add(e);
    }

    fn _padding(&self, message: &[u8]) -> Vec<u8> {
        let mut padded_message = message.to_vec();
        let message_len_bits = (message.len() as u64) * 8;
        
        padded_message.push(0x80);
        while (padded_message.len() * 8) % 512 != 448 {
            padded_message.push(0x00);
        }
        padded_message.extend_from_slice(&message_len_bits.to_be_bytes());
        padded_message
    }

    pub fn hash(&mut self, message: &[u8]) -> String {
        // 修正：每次计算 hash 前应重置为初始状态，否则多次调用 hash 会累加
        let mut context = Sha1::new(); 
        let padded_message = context._padding(message);
        for chunk in padded_message.chunks(64) {
            context._process_block(chunk);
        }
        format!(
            "{:08x}{:08x}{:08x}{:08x}{:08x}",
            context.h0, context.h1, context.h2, context.h3, context.h4
        )
    }
}

