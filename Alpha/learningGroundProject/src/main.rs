use rand::prelude::*;
use std::iter::*;
use std::error::Error;

    /*for i in 1..2 {
        let name = format!("im_blue");
        let path = format!("C:\\Images\\{}.bmp", name);
        let save_path_contrasted = format!("C:\\Images\\{}_contrasted.bmp", name);
        let save_path_swap = format!("C:\\Images\\{}_swap.bmp", name);
        let b = Bitmap::open(&path);
        let mut nb = 0;
        /*for _i in b.get_ptr0().iter() {
            if nb % b.stride == 0 {
                print!("\n");
            }
            print!(" {}", _i);
            nb += 1;
        }*/
        let c = Bitmap::open(&path);

        let d = c.generate_contrast_map();
        let e = c.generate_swapped_color_img();


        /*for _i in d.get_ptr0().iter() {
            if nb % d.stride == 0 {
                print!("\n");
            }
            print!(" {}", _i);
            nb += 1;
        }*/

        d.save(&save_path_contrasted);
        e.save(&save_path_swap);

    }*/


struct Node<'a> {
    links: Vec<Link<'a>>
}

impl<'a> Node<'a> {
    fn new(nodes: &'a [Node]) -> Node<'a> {
        let mut rng = rand::thread_rng();

        let mut links = Vec::new();
        let mut ind = 0;
        for n in nodes {
            links.push(Link::new(rng.gen(), n));
            ind += 1;
        }

        Node { links }
    }
}

struct Link<'a> {
    weigth: f64,
    node: &'a Node<'a>
}

impl<'a> Link<'a> {
    fn new(weigth: f64, node: &'a Node) -> Link<'a> {
        Link { weigth, node }
    }
}

struct Cluster<'a> {
    nodes: Vec<Node<'a>>,
    neighbours: &'a [Cluster<'a>]
}

impl<'a> Cluster<'a> {
    fn new(nodes: Vec<Node<'a>>, neighbours: &'a [Cluster<'a>]) -> Cluster<'a> {
        Cluster { nodes, neighbours }
    }
}

struct NeuralNetwork<'a> {
    clusters: Vec<Cluster<'a>>
}

impl<'a> NeuralNetwork<'a> {
    fn new(clusters: Vec<Cluster<'a>>) -> NeuralNetwork<'a> {
        NeuralNetwork { clusters }
    }
}


struct Matrix<T> {
    data: Vec<Vec<T>>
}


impl Matrix<u8> where
{
    fn from(numbers: &[u8]) -> Matrix<u8> {
        let mut data = Vec::new();
        for n in numbers {
            data.push(Vec::new());

            let mut n_mut = *n;
            for _i in 0..8 {
                data.last_mut()
                .unwrap()
                .push(if n_mut & u8::from(1) != 0 {1} else {0});
                n_mut = n_mut >> 1;
            }
        }
        Matrix { data }
    }

    fn transpose(&self) -> Matrix<u8> {
        let mut data = Vec::new();

        for _i in 0..self.data[0].len() {
            data.push(Vec::new());
            
        }

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                data[i].push(self.data[j][i]);
            }
        }

        Matrix { data }
    }
}

use std::fmt::*;
impl<T> Display for Matrix<T> where
T: Display + Copy
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::new();
        for l in &self.data {
            s.push_str(&format!("|"));
            for n in l {
                s.push_str(&format!(" {}", n));
            }
            s.push_str(&format!(" |\n"));
        }

        write!(f, "{}", s)
    }
}

fn toy_network_exec() {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();

    for _i in 0..8 {
        let val: u8 = rng.gen();
        v.push(val);
    }

    let m: Matrix<u8> = Matrix::from(&v);

    println!("m:\n{}", &m);

    let m_t = m.transpose();
    
    println!("m_t:\n{}", &m_t);
}


fn main() {
    toy_network_exec();
}

use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn benchmark_cast() {
    let vec_u8 = vec![u8::from(1), u8::from(2), u8::from(4), u8::from(8)];
    let vec_u32 = vec![i32::from(1), i32::from(2), i32::from(4), i32::from(8)];

    let time_span = SystemTime::now();
    for _i in 0..10_000_000 {
        let _a = (vec_u8[0] as i32) - (vec_u8[3] as i32);
    }
    println!("u8 -> i32: {:?}", time_span.elapsed().unwrap());

    
    let time_span = SystemTime::now();
    for _i in 0..10_000_000 {
        let _a = vec_u8[3] - vec_u8[0];
    }
    println!("u8 no cast: {:?}", time_span.elapsed().unwrap());

    let time_span = SystemTime::now();
    for _i in 0..10_000_000 {
        let _a = vec_u32[3] - vec_u32[0];
    }
    println!("u32 no cast: {:?}", time_span.elapsed().unwrap());
}


