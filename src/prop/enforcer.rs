use resetprop::PropSystem;
use tracing::{debug, trace, warn};

pub(super) fn enforce_stealth(sys: &PropSystem, props: &[(&str, &str)]) -> (u32, u32) {
    let (mut spoofed, mut skipped) = (0u32, 0u32);
    for &(name, value) in props {
        match sys.get(name) {
            Some(current) if current != value => {
                trace!(prop = name, from = current, to = value, "stealth");
                if sys.set_stealth(name, value).is_ok() {
                    spoofed += 1;
                }
            }
            Some(_) | None => skipped += 1,
        }
    }
    debug!(spoofed, skipped, "enforce_stealth");
    (spoofed, skipped)
}

pub(super) fn set_existing(sys: &PropSystem, names: &[&str], value: &str) -> (u32, u32) {
    let mut changed = 0u32;
    let mut failed = 0u32;
    for &name in names {
        let Some(current) = sys.get(name) else { continue };
        if current == value {
            continue;
        }
        match sys.set_stealth(name, value) {
            Ok(_) => changed += 1,
            Err(e) => {
                failed += 1;
                warn!(prop = name, err = %e, "BRENE67 property write failed");
            }
        }
    }
    (changed, failed)
}

pub(super) fn nuke_props(sys: &PropSystem, names: &[&str]) {
    let (mut nuked, mut absent) = (0u32, 0u32);
    for &name in names {
        if sys.get(name).is_none() {
            absent += 1;
            continue;
        }
        trace!(prop = name, "nuke");
        let result = if name.starts_with("persist.") {
            sys.nuke_persist(name)
        } else {
            sys.nuke(name)
        };
        match result {
            Ok(true) => nuked += 1,
            Ok(false) => absent += 1,
            Err(e) => {
                warn!(prop = name, err = %e, "nuke failed, falling back to hexpatch");
                let _ = sys.hexpatch_delete(name);
                nuked += 1;
            }
        }
    }
    if nuked > 0 {
        debug!(nuked, absent, "nuke_props");
    }
}
