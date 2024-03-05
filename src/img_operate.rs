const SIZES: [u16; 14] = [
    30, 32, 44, 50, 71, 89, 107, 128, 142, 150, 256, 284, 310, 512,
];

pub fn batch_rescale() -> eyre::Result<()> {
    for size in SIZES {
        match size {
            32 => println!("is 32"),
            50 => println!("not 32"),
            128 => println!("not 32"),
            256 => println!("not 32"),
            512 => println!("not 32"),
            _ => println!("not 32"),
        }
    }
    Ok(())
}
