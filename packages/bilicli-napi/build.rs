use cfg_aliases::cfg_aliases;
use napi_build::setup;

fn main() {
    cfg_aliases! {
        napi: {
            feature = "napi"
        }
    }

    setup();
}
