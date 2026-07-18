//! Theme + locale prefs on UiSnapshot.

use lab_slint::UiSnapshot;

#[test]
fn defaults_are_light_acquire_skin_and_english() {
    let snap = UiSnapshot::default();
    assert!(!snap.dark_mode);
    assert_eq!(snap.locale, "en");
    assert!(!snap.demo_seed);
}

#[test]
fn set_dark_mode_round_trips() {
    let mut snap = UiSnapshot::default();
    snap.set_dark_mode(false);
    assert!(!snap.dark_mode);
    snap.set_dark_mode(true);
    assert!(snap.dark_mode);
}

#[test]
fn set_locale_accepts_id_and_en() {
    let mut snap = UiSnapshot::default();
    snap.set_locale("id");
    assert_eq!(snap.locale, "id");
    snap.set_locale("en");
    assert_eq!(snap.locale, "en");
}

#[test]
fn set_locale_coerces_invalid_to_en() {
    let mut snap = UiSnapshot::default();
    snap.set_locale("fr");
    assert_eq!(snap.locale, "en");
    snap.set_locale("");
    assert_eq!(snap.locale, "en");
}
