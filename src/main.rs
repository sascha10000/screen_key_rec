use std::sync::Arc;

use screen_key_rec::*;

fn main() {
    println!("{:?}", RegisterEvents::default());
    /*rec_to_file(
        "yalla",
        RegisterEvents::default(),
        true,
        |k, e| (),
        |m, e| (),
        |k, e| (),
        |k, e| (),
    );*/

    rec_buffer(
        RegisterEvents::default(),
        true,
        Arc::new(|k, e, b| println!("{:?}", k)),
        Arc::new(|k, e, b| println!("{:?}", k)),
    );
}
