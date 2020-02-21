pub fn subgroup_shuffle_xor(caller_tix: u32, values: &[u32; 32], mask: u32) -> u32 {
    let rd_tix = caller_tix ^ mask;
    values[rd_tix as usize]
}

pub fn gen_shifts_and_masks(n: u32) -> (Vec<u32>, Vec<u32>) {
    let mut s = n / 2;
    let mut shifts = Vec::<u32>::new();
    let mut masks = Vec::<u32>::new();

    while s > 0 {
        let mut c = true;
        let mut m: u32 = 0;
        let mut ix = 0;

        for _ in 0..(n / s) {
            if c {
                for j in ix..(ix + s) {
                    m = m | (1 << j as u32);
                }
            }
            ix = ix + s;
            c = !c;
        }
        shifts.push(s);
        masks.push(m);
        s = s / 2;
    }

    (shifts, masks)
}

pub fn shuffle_round(caller_tix: u32, values: &[u32; 32], mask: u32, shift: u32) -> u32 {
    let b = subgroup_shuffle_xor(caller_tix, values, shift);
    let (c, mask) = if ((caller_tix & shift) == 0) {
        (b << shift, mask)
    } else {
        (b >> shift, !mask)
    };
    let r = values[caller_tix as usize];
    (r & mask) | (c & !mask)
}
