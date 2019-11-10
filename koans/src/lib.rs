pub mod enlightenment;
pub mod variables;
pub mod control_flow;
pub mod functions;

#[cfg(test)]
mod tests {
    use super::enlightenment;
    use super::variables;
    use super::control_flow;
    use super::functions;

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
    }
}
