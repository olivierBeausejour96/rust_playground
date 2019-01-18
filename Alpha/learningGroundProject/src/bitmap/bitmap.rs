
use std::fs;

#[derive(Debug)]
pub struct Bitmap {
    width: u32,
    height: u32,
    offset: usize,
    bpp: u32,
    stride: u32,
    data:Box<std::vec::Vec<u8>>
}

impl Bitmap {
    pub fn get_ptr0(&self) -> &[u8] {
        return &self.data[self.offset..]
    }
    pub fn create_from(&self, data: Box<Vec<u8>>) -> Bitmap {
        Bitmap {
            data: data,
            ..*self
        }
    } 
    pub fn open(path: &str) -> Bitmap {
        let ar = fs::read(path).expect("Error reading the file");
        
        let width =
        (ar[21] as u32)<< 24|
        (ar[20] as u32)<< 16| 
        (ar[19] as u32)<< 8 | 
        (ar[18] as u32);
        
        let height =
        (ar[25] as u32)<< 24|
        (ar[24] as u32)<< 16| 
        (ar[23] as u32)<< 8 | 
        (ar[22] as u32);
        
        let offset = (ar[13] as usize) << 24 |  
        (ar[12] as usize)<< 16 |  
        (ar[11] as usize)<< 8 | 
        (ar[10] as usize);
        
        let bpp = 
        (ar[29] as u32 ) << 8 |
        (ar[28] as u32 );
        
        let stride = if bpp * width % 32 == 0 {
            ((bpp * width) / 32) * 4
        }
        else {
            ((bpp * width + 31) /32) * 4
        };

        Bitmap {
            width : width,
            height: height,
            offset: offset,
            bpp: bpp,
            stride: stride,
            data : Box::new(ar)
        }
    }
    pub fn save(&self, path: &str) {
        fs::write(path, &*self.data).unwrap();
    }
    pub fn clone(&self) -> Bitmap {
        Bitmap {
            width: self.width,
            height: self.height,
            offset: self.offset,
            bpp: self.bpp,
            stride: self.stride,
            data: self.data.clone()
        }        
    }
    /*pub fn get_region(&self, rec: &Rectangle) -> std::vec::Vec<&[u8]> {
        let mut v = std::vec::Vec::new();
        for _r in rec.y..rec.y+rec.height {
            v.push(&self.get_ptr0()[_r as usize.._r as usize + rec.width as usize]);
        };
        return v
    }*/
    pub fn distance(&self, bmp: &Bitmap) -> usize {
        let mut val: usize = 0;
        for _i in 0..self.get_ptr0().len() {
            val += 
            if self.get_ptr0()[_i] > bmp.get_ptr0()[_i] {
                ((self.get_ptr0()[_i] - bmp.get_ptr0()[_i]) as usize) *
                ((self.get_ptr0()[_i] - bmp.get_ptr0()[_i]) as usize)
            }
            else {
                ((bmp.get_ptr0()[_i] - self.get_ptr0()[_i]) as usize) *
                ((bmp.get_ptr0()[_i] - self.get_ptr0()[_i]) as usize)
            } 
        }
        val
    }
    pub fn generate_contrast_map(&self) -> Bitmap {
        let mut v = std::vec::Vec::new();
        for y in 0..self.height {
            v.push(std::vec::Vec::new());
            for x in 0..self.width {
                let mut av = 0;
                let mut nb = 0;
                for xi in 3*x..3*x+3 {
                    let p: i32 = self.get_ptr0()[(y as usize) * (self.stride as usize) + (xi as usize)].into();
                    if x > 0 {
                        nb += 1;
                        let po: i32 = self.get_ptr0()[(y as usize) * (self.stride as usize) + ((xi - 3) as usize)].into();

                        let diff: i32 = p - po;
                        av += diff * diff;
                    }

                    if y > 0 {
                        nb += 1;
                        let pn: i32 = self.get_ptr0()[((y - 1) as usize) * (self.stride as usize) + (xi as usize)].into();

                        let diff: i32 = p - pn;
                        av += diff * diff;
                    }

                    if x < self.width - 1 {
                        nb += 1;
                        let pe: i32 = self.get_ptr0()[(y as usize) * (self.stride as usize) + ((xi + 3) as usize)].into();

                        let diff: i32 = p - pe;
                        av += diff * diff;
                    }

                    if y < self.height - 1 {
                        nb += 1;
                        let ps: i32 = self.get_ptr0()[((y + 1) as usize) * (self.stride as usize) + (xi as usize)].into();

                        let diff: i32 = p - ps;
                        av += diff * diff;
                    }
                }
                av /= nb;
                v[(y as usize)].push(av);
            }
        }

        let mut new_data = *self.data.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut vals = std::vec::Vec::new();
                if x > 0 {
                    vals.push(v[(y as usize)][((x - 1) as usize)])
                }

                if y > 0 {
                    vals.push(v[((y - 1) as usize)][(x as usize)]);
                }

                if x < self.width - 1 {
                    vals.push(v[(y as usize)][((x + 1) as usize)])
                }

                if y < self.height - 1 {
                    vals.push(v[((y + 1) as usize)][(x as usize)]);
                }
                vals.sort();
                let ratio = *vals.last().unwrap() / (*vals.first().unwrap() + 1);
                let px = (x as usize) * 3 + (y as usize) * (self.stride as usize) + self.offset;
                if ratio >= 8 {
                    //println!("ratio: {}", ratio);
                    new_data[px] = 0;
                    new_data[px+1] = 0;
                    new_data[px+2] = 0;
                }
                else {
                    new_data[px] = 255;
                    new_data[px+1] = 255;
                    new_data[px+2] = 255;
                }
            }
        }

        self.create_from(Box::new(new_data))
    }

    pub fn generate_swapped_color_img(&self) -> Bitmap {
        let mut new_data = *self.data.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let px = (x as usize) * 3 + (y as usize) * (self.stride as usize) + self.offset;
                let temp = new_data[px];
                new_data[px] = new_data[px + 2];
                new_data[px+1] = new_data[px + 1]; 
                new_data[px+2] = temp;
            }
        }

        self.create_from(Box::new(new_data))
    }

    pub fn generate_more_green(&self) -> Bitmap {
        let mut new_data = *self.data.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let px = (x as usize) * 3 + (y as usize) * (self.stride as usize) + self.offset;
                let _temp = new_data[px];
                new_data[px+1] = new_data[px+1];
            }
        }

        self.create_from(Box::new(new_data))
    }
}
