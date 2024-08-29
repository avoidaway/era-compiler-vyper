use crate::{cli, common};
use predicates::prelude::*;

/// id1939
#[test]
fn run_with_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--disable-vyper-optimizer"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1939:I
#[test]
fn run_only_with_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--disable-vyper-optimizer"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "the following arguments are required",
    ));

    Ok(())
}

/// id1979
#[test]
fn run_with_duplicate_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
        cli::TEST_VYPER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}