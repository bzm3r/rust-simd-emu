mod bitmat;
mod simd_emu;

fn main() {
    let bm = bitmat::BitMatrix::new_corner();
    let tbm = bm.transpose_simd_emu();

    println!("bm: {}", bm);
    println!("tbm: {}", tbm);
    assert!(bm.transpose().identical_to(&tbm));
}
