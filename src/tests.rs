use proptest::prelude::*;
use proptest::strategy::ValueFor;
use proptest::test_runner::{Config, TestCaseResult, TestError, TestRunner};

pub(crate) fn run_test<S: Strategy, F: Fn(&ValueFor<S>) -> TestCaseResult>(
    strategy: &S,
    test: F,
    source_file: &'static str
) -> Result<(), TestError<ValueFor<S>>> {
    let mut runner = TestRunner::new(Config {
        source_file: Some(source_file),
        .. Default::default()
    });
    runner.run(strategy, test)
}
