//by default the child module is private
pub mod hosting;

pub mod serving {
    pub fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}