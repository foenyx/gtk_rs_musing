// Random musing about IconView using cairo-customized Pixbuf in Gtk-rs 0.1.1

extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate glib;
extern crate cairo;

use self::gtk::prelude::*;
use self::gtk::{ WindowPosition, Window, WindowType, ListStore, IconView, Align,};
use self::gdk::prelude::ContextExt;
use self::gdk_pixbuf::Pixbuf;
use self::cairo::enums::{FontSlant, FontWeight};
use self::cairo::{Context, Format, ImageSurface, ImageSurfaceData, TextExtents};

pub fn load_img(filename: &str) -> Pixbuf {
	match Pixbuf::new_from_file(filename) {
		Err(e) => {
	        println!("Err: {}.", e);
	        panic!("Pixbuf::new_from_file failed!");
	    },
	    Ok(p) => p
	}
}

const FONT_SIZE : f64 = 48.;
const FONT_NAME : &'static str = "Symbola";

pub struct Color {
	r : u8,
	g : u8,
	b : u8,
	a : u8
}

impl Color {
	fn rgba(red:u8, green:u8, blue:u8, alpha:u8) -> Color {
		Color {r:red, g:green, b:blue, a:alpha}
	}
	
	fn rgb(red:u8, green:u8, blue:u8) -> Color {
		Color::rgba(red, green, blue, 255)
	}
	
	fn as_f64_tuple(&self) -> (f64, f64, f64, f64) {
		(
			self.r as f64 / 255.,
			self.g as f64 / 255.,
			self.b as f64 / 255.,
			self.a as f64 / 255.,	
		)
	}
}

pub struct Dimension{
	width: f64,
	height: f64
}

impl Dimension {
	fn new(width: i32, height: i32) -> Dimension {
		Dimension { width: width as f64, height: height as f64 }
	}
}

pub fn draw_text(cr : &Context, bounding_box: &Dimension, text : &str, color : &Color){

	let (r, g, b, a) = color.as_f64_tuple();
	cr.set_source_rgba(r, g, b, a);
	cr.select_font_face(FONT_NAME, FontSlant::Normal, FontWeight::Normal);
	cr.set_font_size(FONT_SIZE);

	// centering text
	let txt_ext : TextExtents = cr.text_extents(text);
	let (w, h) = (txt_ext.width, txt_ext.height);
	cr.move_to(
		((bounding_box.width- w) / 2.), 
		bounding_box.height - (((bounding_box.height - h) / 2.))
	);
	
	// text inner filling
	cr.text_path(text);
	cr.fill_preserve();
	
	// text gray outline
	cr.set_source_rgba (0.2, 0.2, 0.2, 0.75);
	cr.set_line_width (1.);
	cr.stroke();
}

// Take a pixbuf as input model, and create a same dimensions Pixbuf that can be modified with cairo
pub fn custom_icon<F>(bg : &Pixbuf, cairo_processing : F ) -> Pixbuf 
	where F : Fn(&Context, &Dimension)
{
	
	// Inspired from https://gist.github.com/bert/985903 
	// and adapted to gtk-rs 0.1.1 available features :
	// 
	// let surf = ImageSurface::create();
	// let cr = Context::new(&surf);
	// painting on cairo context as wanted 
	// let rpixbuf = Pixbuf::new()
	// transfering surf.get_data() to rpixbuf.put_pixel()  
	
	let (width, height)	= (bg.get_width(), bg.get_height());
	let has_alpha 		= bg.get_has_alpha();
	let bits_per_sample	= bg.get_bits_per_sample();
	let colorspace 		= bg.get_colorspace();
	
	let mut surf : ImageSurface = ImageSurface::create(Format::ARgb32, width, height);  
	{
		let cr = Context::new(&surf);
		cairo_processing(&cr, &Dimension::new(width, height));		
	}
	
	let data : ImageSurfaceData = match surf.get_data() 
	{
		Err(e) => {
	        println!("Err: {:?}.", e);
	        panic!("ImageSurfaceData.get_data failed!");
	    },
	    Ok(d) => d
	};

	let rpixbuf : Pixbuf = match unsafe { Pixbuf::new(colorspace, has_alpha, bits_per_sample, width, height) }
	{
			Ok(p) => p,
			_ => panic!("Pixbuf::new failed!")
	};
	
	// Format::ARgb32 in reverse order [b,g,r,a] ?
	// or maybe endianess dependant?
	const SURF_BLUE_CHAN 	: usize = 0;
	const SURF_GREEN_CHAN 	: usize = 1;
	const SURF_RED_CHAN 	: usize = 2;
	const SURF_ALPHA_CHAN 	: usize = 3;
	
	for (i,chunk) in data.chunks(4).enumerate() {
		let x = (i as i32) % width;
		let y = (i as i32) / width;
		
		rpixbuf.put_pixel(
			x, y,
			chunk[SURF_RED_CHAN],
			chunk[SURF_GREEN_CHAN],
			chunk[SURF_BLUE_CHAN],
			chunk[SURF_ALPHA_CHAN], 
		);
	}

	rpixbuf	
}

fn create_and_fill_liststore() -> ListStore {
	
	// Loading a "tile" background image for the icons     
    let bg : Pixbuf = load_img("resources/background-64px.png"); // 64px²
    
    // a rainbowish palette                
	let colors = [
		Color::rgb(0xFF, 	0, 		0),
		Color::rgb(0xFF, 	0xA5,	0),
		Color::rgb(0xFF, 	0xFB,	0),
		Color::rgb(0, 		0xFF, 	0),
		Color::rgb(0, 		0, 		0xFF),
		Color::rgb(0x80,	0, 		0x80),
	]; 
	let mut color_it = colors.iter().cycle();
	
	// The face of the tiles 
	let tiles = "a b c d e f 1 2 3 4 5 6 ♥ ♦ ★ ♣ ⚫ ♠";
	
    let model = ListStore::new(&[Pixbuf::static_type()]);
	for char in tiles.chars() {
		if char != ' ' {
			let text = format!("{}", char);
			let color = color_it.next().unwrap();
			
			let pixbuf_icon = custom_icon(&bg, |cr, dim| {
				// painting the background image pixbuf	
				cr.set_source_pixbuf(&bg, 0., 0.);
				cr.paint();
				
				// Custom cairo painting over the background  	
				draw_text(&cr, &dim, &text, color);
			});
			
	        model.insert_with_values(None, &[0], &[ &pixbuf_icon]);
		}
	}
	
	model
}

pub fn main() {
	
	if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }
        
	let liststore_model = create_and_fill_liststore();

	let window = Window::new(WindowType::Toplevel);
	
    window.set_title("Simple Gtk-rs Pixbuf IconView Example");
	window.set_position(WindowPosition::Center);
	window.set_size_request(512, 128);
	window.set_default_size(512, 128);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
	
	let box_container = gtk::Box::new(gtk::Orientation::Vertical, 1);
	
	let icon_view = IconView::new();
	icon_view.set_valign(Align::Fill);	
	icon_view.set_reorderable(true);
	icon_view.set_model(Some(&liststore_model));
	icon_view.set_pixbuf_column(0);
	box_container.add(&icon_view);
	
	window.add(&box_container);
	
    window.show_all();
    gtk::main();
}
