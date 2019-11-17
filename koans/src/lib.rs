pub mod enlightenment;
pub mod variables;
pub mod control_flow;
pub mod functions;
pub mod data_types;

#[cfg(test)]
mod tests {
    use super::enlightenment;
    use super::variables;
    use super::control_flow;
    use super::functions;
    use super::data_types;

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
    }
}
