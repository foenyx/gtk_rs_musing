# gtk-rs musing
Random experimentations with [rust](https://www.rust-lang.org/), [gtk-rs](http://gtk-rs.org) and [cairo](https://www.cairographics.org) :

 * [pixbuf_iconview](https://github.com/foenyx/gtk_rs_musing/blob/master/src/pixbuf_iconview.rs) 
 ([screenshot](https://github.com/foenyx/gtk_rs_musing/blob/master/screenshots/pixbuf_iconview.png)) :
 populating a [`gtk::IconView`](http://gtk-rs.org/docs/gtk/struct.IconView.html) widget
 with some [`gdk_pixbuf::Pixbuf`](http://gtk-rs.org/docs/gdk_pixbuf/struct.Pixbuf.html).
 The Pixbufs are generated from a `.png` background and some 
 [`cairo::Context`](http://gtk-rs.org/docs/cairo/struct.Context.html) rendering.
 
 
 
