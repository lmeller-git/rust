use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, RelroLevel, StackProbeType, TargetOptions,
};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "tinyos".into(),
        disable_redzone: true,
        has_thread_local: false,
        panic_strategy: PanicStrategy::Abort,
        plt_by_default: false,
        stack_probes: StackProbeType::Inline,
        position_independent_executables: false,
        static_position_independent_executables: true,
        relro_level: RelroLevel::Full,
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        relocation_model: RelocModel::Static,
        pre_link_args: TargetOptions::link_args(
            LinkerFlavor::Gnu(Cc::No, Lld::No), // will also insert Lld::Yes
            &vec!["-nostdlib".into(), "-static".into(), "-no-pie".into(), "--gc-sections".into()],
        ),
        crt_static_respected: true,
        ..Default::default()
    }
}
