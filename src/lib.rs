pub mod verilog;
pub mod testcase;
pub mod simplify;
pub mod ebnf_tools;
pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
