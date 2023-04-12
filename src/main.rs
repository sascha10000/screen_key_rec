use std::sync::Arc;

use screen_key_rec::*;

fn main() {
    rec_on_key("yalla", true, |k, e| (), |m, e| (), |k,e| (), |k, e| ());
}
