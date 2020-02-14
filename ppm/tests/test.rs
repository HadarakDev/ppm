// use ppm;
use std::path::Path;
extern crate ppm;


#[test]
fn test_create_pixel() {
    let p = ppm::Pixel::new(0, 0, 0);

    assert_eq!(p.red(), 0);
    assert_eq!(p.green(), 0);
    assert_eq!(p.blue(), 0);
}

#[test]
fn test_invert_pixel() {
    let mut p = ppm::Pixel::new(0, 0, 0);

    p.invert();
    assert_eq!(p.red(), 255);
    assert_eq!(p.green(), 255);
    assert_eq!(p.blue(), 255);  
}

#[test]
fn test_display_pixel() {
    let p = ppm::Pixel::new(0, 0, 0);

    assert_eq!(format!("{}", p), "r = 0, g = 0, b = 0"); 
}

#[test]
fn test_eq_pixel() {
    let p1 = ppm::Pixel::new(0, 0, 0);
    let p2 = ppm::Pixel::new(0, 0, 0);

    assert!(p1 == p2);
}


#[test]
fn test_gray_true() {
    let mut p1 = ppm::Pixel::new(255, 0, 167);
    
    p1.true_gray_scale();
    assert_eq!(p1.red(), 94);
    assert_eq!(p1.green(), 94);
    assert_eq!(p1.blue(), 94);
}

#[test]
fn test_gray_basic() {
    let mut p1 = ppm::Pixel::new(255, 0, 167);
    
    p1.basic_gray_scale();
    assert_eq!(p1.red(), 140);
    assert_eq!(p1.green(), 140);
    assert_eq!(p1.blue(), 140);
}

#[test]
fn test_open_image() {
    let path = Path::new("img.ppm");
    ppm::Image::new_with_file(path);
}

#[test]
fn test_save_image() {
    let path = Path::new("img.ppm");
    let path2 = Path::new("img2.ppm");
    let _img = ppm::Image::new_with_file(path);
    _img.save_to_ppm(path2);

}

#[test]
fn test_load_image_saved() {
    let path = Path::new("img2.ppm");
    ppm::Image::new_with_file(path);
}

#[test]
fn test_display_Image() {
    let path = Path::new("img2.ppm");
    let img = ppm::Image::new_with_file(path);
    println!("{}", img); 
}

#[test]
fn  test_true_convert_to_gray()
{
    let path = Path::new("img2.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.convert_image_to_gray(0);
    println!("{}", img); 
}

#[test]
fn  test_basic_convert_to_gray()
{
    let path = Path::new("img.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.convert_image_to_gray(1);
    println!("{}", img); 
    let path_gray = Path::new("imggray.ppm");
    img.save_to_ppm(path_gray)
}

#[test]
fn  test_invert_image()
{
    let path = Path::new("img2.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.invert();
    println!("{}", img); 
}

#[test]
fn  test_rotate_180()
{
    let path = Path::new("img2.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.display_image_in_terminal();
    img.rotate_180();
    img.display_image_in_terminal();
}

#[test]
fn  test_rotate_90()
{
    let path = Path::new("img3.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.display_image_in_terminal();
    img.rotate_90();
    img.display_image_in_terminal();
}

#[test]
fn  test_rotate_270()
{
    let path = Path::new("img3.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.display_image_in_terminal();
    img.rotate_270();
    img.display_image_in_terminal();
}

#[test]
fn  test_flip_horizontal()
{
    let path = Path::new("img2.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.display_image_in_terminal();
    img.flip_horizontal();
    img.display_image_in_terminal();
}

#[test]
fn  test_flip_vertical()
{
    let path = Path::new("img2.ppm");
    let mut img = ppm::Image::new_with_file(path);
    img.display_image_in_terminal();
    img.flip_vertical();
    img.display_image_in_terminal();
    
}

#[test]   
fn  test_convert_png()
{
    let path = Path::new("images/resize/img3.png");
    let mut img = ppm::load_png_as_ppm(path);
    img.display_image_in_terminal();
}