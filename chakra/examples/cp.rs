use chakra::{self, Flags, IoRing};

fn main() {
    let (ring, params) =
        IoRing::init_params(256, Flags::empty()).expect("Unable to instantiate ring");
}
