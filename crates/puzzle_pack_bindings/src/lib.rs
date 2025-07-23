pub mod ts_api;

mod layers {
    pub(super) mod first_layer {
        pub(super) struct Rule {
        }
        pub(super) struct Deduction {}
        pub(super) struct LogicStep {}
    }
    pub(super) mod second_layer;
}
