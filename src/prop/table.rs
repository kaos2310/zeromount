pub(super) struct PropEntry {
    pub name: &'static str,
    pub value: &'static str,
}

// Guard5/BRENE67 general group. The ten boot/AVB properties are intentionally
// excluded here: src/boot_values.rs is their single writer.
pub(super) static GENERAL: &[PropEntry] = &[
    PropEntry { name: "ro.debuggable", value: "0" },
    PropEntry { name: "ro.secure", value: "1" },
    PropEntry { name: "ro.build.type", value: "user" },
    PropEntry { name: "ro.build.tags", value: "release-keys" },
    PropEntry { name: "ro.adb.secure", value: "1" },
    PropEntry { name: "ro.crypto.state", value: "encrypted" },
    PropEntry { name: "ro.force.debuggable", value: "0" },
    PropEntry { name: "ro.kernel.qemu", value: "" },
    PropEntry { name: "ro.secureboot.lockstate", value: "locked" },
    PropEntry { name: "ro.is_ever_orange", value: "0" },
    PropEntry { name: "ro.bootmode", value: "normal" },
    PropEntry { name: "ro.bootimage.build.tags", value: "release-keys" },
    PropEntry { name: "ro.boot.realme.lockstate", value: "1" },
    PropEntry { name: "ro.boot.realmebootstate", value: "green" },
    PropEntry { name: "ro.boot.verifiedbooterror", value: "" },
    PropEntry { name: "ro.boot.veritymode.managed", value: "yes" },
    PropEntry { name: "ro.boot.vbmeta.hash_alg", value: "sha256" },
    PropEntry { name: "ro.boot.vbmeta.avb_version", value: "1.3" },
    PropEntry { name: "ro.boot.vbmeta.invalidate_on_error", value: "yes" },
    PropEntry { name: "sys.oem_unlock_allowed", value: "0" },
];

pub(super) static FINGERPRINT_PROPS: &[&str] = &[
    "ro.bootimage.build.fingerprint",
    "ro.build.fingerprint",
    "ro.odm.build.fingerprint",
    "ro.odm_dlkm.build.fingerprint",
    "ro.product.build.fingerprint",
    "ro.system.build.fingerprint",
    "ro.system_dlkm.build.fingerprint",
    "ro.system_ext.build.fingerprint",
    "ro.vendor.build.fingerprint",
    "ro.vendor_dlkm.build.fingerprint",
];

pub(super) static UTC_PROPS: &[&str] = &[
    "ro.bootimage.build.date.utc",
    "ro.build.date.utc",
    "ro.odm.build.date.utc",
    "ro.odm_dlkm.build.date.utc",
    "ro.product.build.date.utc",
    "ro.system.build.date.utc",
    "ro.system_dlkm.build.date.utc",
    "ro.system_ext.build.date.utc",
    "ro.vendor.build.date.utc",
    "ro.vendor_dlkm.build.date.utc",
    "persist.vendor.build.date.utc",
];

pub(super) static DATE_PROPS: &[&str] = &[
    "ro.bootimage.build.date",
    "ro.build.date",
    "ro.odm.build.date",
    "ro.odm_dlkm.build.date",
    "ro.product.build.date",
    "ro.system.build.date",
    "ro.system_dlkm.build.date",
    "ro.system_ext.build.date",
    "ro.vendor.build.date",
    "ro.vendor_dlkm.build.date",
];

// Props that leak PIF module presence
pub(super) static NUKE_PIF: &[&str] = &[
    "persist.sys.pihooks.status",
    "persist.sys.pihooks",
    "ro.pihooks.enable",
    "persist.pihooks.mainline_update",
    "persist.sys.pixelprops.pi",
    "persist.sys.pixelprops.gms",
    "persist.sys.pixelprops.gphotos",
    "persist.sys.pixelprops.netflix",
];

// Props that leak custom ROM identity
pub(super) static NUKE_CUSTOM_ROM: &[&str] = &[
    "ro.lineage.build.version",
    "ro.lineage.build.version.plat_sdk",
    "ro.lineage.version",
    "ro.lineage.display.version",
    "ro.lineage.releasetype",
    "ro.lineageaudio.version",
    "ro.crdroid.build.version",
    "ro.crdroid.version",
    "ro.crdroid.display.version",
    "ro.modversion",
    "ro.romversion",
    "ro.rom.build.display.id",
    "ro.custom.build.version",
];
