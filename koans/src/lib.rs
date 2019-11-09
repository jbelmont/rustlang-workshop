pub mod enlightenment;
pub mod variables;
pub mod control_flow;

#[cfg(test)]
mod tests {
    use super::enlightenment;
    use super::variables;
    use super::control_flow;

    #[test]
    fn koan_journey() {
        enlightenment::basic_assertion();
        variables::variables();
        control_flow::control_flow();
    }
}
