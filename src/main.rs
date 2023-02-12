use image_ascii_converter::ASCIIConverter;

fn main() {
    println!("Hello, world!");
    let converter = ASCIIConverter::new("dummy.jpg", "output.png", 32, 1);
    converter.convert_to_img().unwrap();
    // let result = converter.convert_to_str().unwrap();
    // println!("{}", result);
}
