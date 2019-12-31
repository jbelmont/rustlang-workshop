pub mod enlightenment;
pub mod variables;
pub mod control_flow;
pub mod functions;
pub mod data_types;
pub mod ownership;
pub mod borrowing;
pub mod slices;
pub mod structs;
pub mod methods;
pub mod enums;
pub mod patterns;
pub mod modules;
pub mod vectors;
pub mod strings;
pub mod hash_maps;
pub mod error_handling;
pub mod generic_types;
pub mod traits;
pub mod lifetimes;

#[cfg(test)]
mod tests {
    use super::enlightenment;
    use super::variables;
    use super::control_flow;
    use super::functions;
    use super::data_types;
    use super::ownership;
    use super::borrowing;
    use super::slices;
    use super::structs;
    use super::methods;
    use super::enums;
    use super::patterns;
    use super::modules;
    use super::vectors;
    use super::strings;
    use super::hash_maps;
    use super::error_handling;
    use super::generic_types;
    use super::traits;
    use super::lifetimes;

    #[test]
    fn koan_journey() {
        // enlightenment.rs
        enlightenment::basic_assertion();

        // variables.rs
        variables::variables();

        // control_flow.rs
        control_flow::control_flow();

        // functions.rs
        functions::functions();

        // data_types.rs
        data_types::data_types();

        // ownership.rs
        ownership::ownership();

        // borrowing.rs
        borrowing::borrowing();

        // slices.rs
        slices::slices();

        // structs.rs
        structs::structs();

        // methods.rs
        methods::methods();

        // enums.rs
        enums::enums();

        // patterns.rs
        patterns::patterns();

        // modules.rs
        modules::usage::example::logging();

        // vectors.rs
        vectors::vectors();

        // strings.rs
        strings::strings();

        // hash_maps.rs
        hash_maps::hash_maps();

        // error_handling.rs
        error_handling::error_handling();

        // generic_types.rs
        generic_types::generic_types();

        // traits.rs
        traits::traits();

        // lifetimes.rs
        lifetimes::lifetimes();
    }
}
