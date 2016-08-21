extern crate systemd;

use systemd::login;
use systemd::daemon::booted;

#[test]
fn test_get_unit() {
    let uu = login::get_unit(login::UnitType::UserUnit, None);
    let su = login::get_unit(login::UnitType::SystemUnit, None);
    let has_systemd = booted();
    assert!(has_systemd.is_ok());
    match has_systemd.unwrap() {
        // This is not running in an unit at all
        false => { assert!(uu.is_err()); assert!(su.is_err()); },
        // This is either running in a system or in an user unit
        true => { assert_eq!(uu.is_err(), su.is_ok()); },
    };
}

#[test]
fn test_get_slice() {
    let us = login::get_slice(login::UnitType::UserUnit, None);
    let ss = login::get_slice(login::UnitType::SystemUnit, None);
    let has_systemd = booted();
    assert!(has_systemd.is_ok());
    match has_systemd.unwrap() {
        // This is running in the top-level generic slice
        false => { assert_eq!(ss.unwrap(), "-.slice"); },
        // This is running in a system slice, and perhaps
        // in an user one too
        true => { assert!(ss.is_ok() || us.is_ok()); },
    };
}
