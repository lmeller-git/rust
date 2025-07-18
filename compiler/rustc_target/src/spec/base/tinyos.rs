use crate::spec::TargetOptions;

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "tinyos".into(),
        disable_redzone: true,
        has_thread_local: false,
        ..Default::default()
    }
}
