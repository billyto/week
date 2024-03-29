use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs


#[test]
fn todays_week() -> Result<(), Box<dyn std::error::Error>> {


    let mut cmd = Command::cargo_bin("week")?;
    cmd.assert().success().stdout(predicate::str::is_match("Is weeek \\d{1,2}\\n").unwrap()); //.stdout(predicate::str::is_match("[0-9][0-9]"));
    Ok(())
}


#[test]
fn date_week() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("19-02-2023");
    cmd.assert().success().stdout(predicate::str::contains("Is weeek 7"));
    Ok(())
}


#[test]
fn yearless_week() -> Result<(), Box<dyn std::error::Error>> {


    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("01/01");
    cmd.assert().success().stdout(predicate::str::is_match("Is weeek \\d{1,2}\\n").unwrap()); //.stdout(predicate::str::is_match("[0-9][0-9]"));
    Ok(())
}

#[test]
fn invalid_date() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("202302/19");
    cmd.assert().failure();
    Ok(())

}