use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use anyhow::Result; // Friendlier error management



#[test]
fn test_current_week_output() -> Result<()> {
    let mut cmd = Command::cargo_bin("week")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^Is weeek \d{1,2}\n$")?);
    Ok(())
}

#[test]
fn date_week() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("19-02-2023");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Is weeek 7"));
    Ok(())
}

#[test]
fn test_specific_date_formats() -> Result<()> {
    let test_cases = vec![
        ("01-01-2024", "1"),  // First week of 2024
        ("01/01/2024", "1"),  // Same date, different format
        ("15/03/2024", "11"), // Mid-March 2024
        //("31.12.2023", "52"), // Last week of 2023 // TODO: Fix this tests, depends on the calendar
    ];

    for (input_date, expected_week) in test_cases {
        let mut cmd = Command::cargo_bin("week")?;
        cmd.arg("--date").arg(input_date)
            .assert()
            .success()
            .stdout(format!("Is weeek {}\n", expected_week));
    }
    Ok(())
}

#[test]
fn test_invalid_date_formats() -> Result<()> {
    let invalid_dates = vec![
        "invalid",
        "32-01-2024",  // Invalid day
        "01-13-2024",  // Invalid month
        "2024-01-01",  // Wrong format order
        "",            // Empty string
    ];

    for invalid_date in invalid_dates {
        let mut cmd = Command::cargo_bin("week")?;
        cmd.arg("--date").arg(invalid_date)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Failed to parse date"));
    }
    Ok(())
}

#[test]
fn yearless_week() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("01/01");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("Is weeek \\d{1,2}\\n").unwrap()); //.stdout(predicate::str::is_match("[0-9][0-9]"));
    Ok(())
}

#[test]
fn invalid_date() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("week")?;
    cmd.arg("--date").arg("202302/19");
    cmd.assert().failure();
    Ok(())
}
