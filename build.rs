use pipl::*;
fn main() {
    for name in [
        "does_dialog",
        "with_premiere",
        "threaded_rendering",
        "catch_panics",
    ] {
        println!("cargo:rustc-check-cfg=cfg({name})");
    }
    println!("cargo:rustc-cfg=catch_panics");
    println!("cargo:rerun-if-env-changed=GLITCHTRACE_BUILD_SHA");
    println!(
        "cargo:rustc-env=GLITCHTRACE_BUILD_SHA={}",
        std::env::var("GLITCHTRACE_BUILD_SHA").unwrap_or_else(|_| "local".into())
    );
    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("GlitchTrace"),
        Property::Category("Nasaat"),
        #[cfg(target_os = "windows")]
        Property::CodeWin64X86("EffectMain"),
        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version {
            major: 13,
            minor: 28,
        },
        Property::AE_Effect_Version {
            version: 1,
            subversion: 0,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        // Detection depends on the whole image. Never advertise PixIndependent.
        Property::AE_Effect_Global_OutFlags(OutFlags::DeepColorAware),
        // MFR, GPU and float/SmartFX are deliberately not advertised without host validation.
        Property::AE_Effect_Global_OutFlags_2(OutFlags2::None),
        Property::AE_Effect_Match_Name("Nasaat GlitchTrace"),
        Property::AE_Reserved_Info(8),
        Property::AE_Effect_Support_URL("https://github.com/NasaatR/Glitch_Trace/issues"),
    ]);
}
