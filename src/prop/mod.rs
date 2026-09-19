mod enforcer;
mod table;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use resetprop::PropSystem;
use tracing::{info, warn};

use crate::core::config::ZeroMountConfig;

fn sanitize_fingerprint(value: &str) -> String {
    value
        .replace("userdebug", "user")
        .replace("evolution", "")
        .replace("crdroid", "")
        .replace("lineage", "")
}

fn apply_general(sys: &PropSystem, config: &ZeroMountConfig) -> (u32, u32) {
    enforcer::nuke_props(sys, table::NUKE_PIF);
    enforcer::nuke_props(sys, table::NUKE_CUSTOM_ROM);

    let props: Vec<(&str, &str)> = table::GENERAL.iter().map(|p| (p.name, p.value)).collect();
    let (mut applied, mut failed) = enforcer::enforce_stealth(sys, &props);

    let size = config.brene.vbmeta_size.to_string();
    if let Some(current) = sys.get("ro.boot.vbmeta.size") {
        if current != size {
            match sys.set_stealth("ro.boot.vbmeta.size", &size) {
                Ok(_) => applied += 1,
                Err(_) => failed += 1,
            }
        }
    }

    (applied, failed)
}

fn apply_identity_groups(sys: &PropSystem, config: &ZeroMountConfig) -> (u32, u32) {
    let mut applied = 0u32;
    let mut failed = 0u32;

    if config.brene.spoof_fingerprint_properties {
        if let Some(fp) = sys.get("ro.build.fingerprint") {
            let fp = sanitize_fingerprint(&fp);
            let (a, f) = enforcer::set_existing(sys, table::FINGERPRINT_PROPS, &fp);
            applied += a;
            failed += f;
        }
    }

    if config.brene.spoof_utc_properties {
        if let Some(utc) = sys.get("ro.build.date.utc") {
            let (a, f) = enforcer::set_existing(sys, table::UTC_PROPS, &utc);
            applied += a;
            failed += f;
        }
    }

    if config.brene.spoof_date_properties {
        if let Some(date) = sys.get("ro.build.date") {
            let (a, f) = enforcer::set_existing(sys, table::DATE_PROPS, &date);
            applied += a;
            failed += f;
        }
    }

    (applied, failed)
}

pub fn run_prop_watch(repeat: bool) -> Result<()> {
    let config = ZeroMountConfig::load(None)?;

    if !config.brene.prop_spoofing {
        info!("prop-watch: prop spoofing disabled, exiting");
        return Ok(());
    }

    let sys = match PropSystem::open() {
        Ok(s) => s,
        Err(e) => {
            warn!("prop-watch: cannot open property areas: {e}");
            return Ok(());
        }
    };

    let (general_applied, general_failed) = apply_general(&sys, &config);
    let (identity_applied, identity_failed) = apply_identity_groups(&sys, &config);
    info!(
        "BRENE 67 identity properties applied={} failed={}",
        general_applied + identity_applied,
        general_failed + identity_failed
    );

    if !repeat {
        return Ok(());
    }

    // Guard5 semantics: only the general property group repeats every minute.
    loop {
        for _ in 0..60 {
            if crate::utils::signal::shutdown_requested() {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(1));
        }

        let config = ZeroMountConfig::load(None)?;
        if !config.brene.prop_spoofing || !config.brene.prop_spoofing_repeat {
            info!("prop-watch: repeat disabled, exiting");
            return Ok(());
        }
        let (applied, failed) = apply_general(&sys, &config);
        info!("BRENE 67 repeat properties applied={applied} failed={failed}");
    }
}
