use std::convert::TryInto;

pub fn both(content: &str, num_iter: usize) -> u16 {
    let mut split = content.split("\n\n");
    let algorithm = split.next().unwrap().chars().map(|x|(x=='#') as u16).collect::<Vec<u16>>();
    let image_str = split.next().unwrap();

    const SIZE: usize = 100;
    const PAD: usize = 5*SIZE;
    let mut image = [[0;SIZE+2*PAD];SIZE+2*PAD];
    let pad_str = ".".repeat(PAD);
    for (i,line) in image_str.split("\n").enumerate() {
        let char_iter = format!("{}{}{}",pad_str,line,pad_str).chars().map(|x|(x=='#') as u16).collect::<Vec<u16>>();
        image[i+PAD] = char_iter.try_into().unwrap(); 
    }

    for _ in 0..num_iter {
        // it might be the case that the algorithm dictates a background inversion each step
        // this is the case if the algorithm at offset 0x0 returns a 1 and at offset 0x1FF (9xones) returns 0
        // if this is the case: invert the background on each iteration
        // if not: just clone the old image
        //
        // we have to do this as no matter how large the padding is, the outer border cannot be translated by a traditional stencil
        let mut new_image = match (algorithm[0b000000000], algorithm[0b111111111]) {
            (1,0) => [[(image[0][0] == 0) as u16;SIZE+2*PAD];SIZE+2*PAD],
            _ => image.clone(),
        };
        for i in 1..SIZE+2*PAD-1 {
            for j in 1..SIZE+2*PAD-1 {
                let num = (image[i-1][j-1] << 8) | 
                          (image[i-1][j  ] << 7) |
                          (image[i-1][j+1] << 6) |
                          (image[i  ][j-1] << 5) |
                          (image[i  ][j  ] << 4) |
                          (image[i  ][j+1] << 3) |
                          (image[i+1][j-1] << 2) |
                          (image[i+1][j  ] << 1) |
                          (image[i+1][j+1] << 0)
                          ;
                new_image[i][j] = algorithm[num as usize];

            }
        }
        // for i in 0..SIZnum_iter
        image = new_image;
    }

    image.iter().fold(0,|acc,x| acc + x.iter().fold(0,|acc2,y| acc2+y))
}

