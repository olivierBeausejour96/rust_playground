extern crate bitmap;


fn main() {
    let path = String::from("C:\\Images\\");
    let img = String::from("im_blue_swap");
    let out = String::from("{}out\\", path);
    let ext = String::from(".bmp");
    let full_path = format!("{}{}{}", path, img, ext);

    let bmp = bitmap::Bitmap::open(&full_path);

}