// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1/unit/ssh_2Eservice -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Unit`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Unit {
    fn start(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn stop(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn try_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload_or_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn reload_or_try_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn enqueue_job(&self, arg0: &str, arg1: &str) -> Result<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String, Vec<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String)>), dbus::Error>;
    fn kill(&self, arg0: &str, arg1: i32) -> Result<(), dbus::Error>;
    fn reset_failed(&self) -> Result<(), dbus::Error>;
    fn set_properties(&self, arg0: bool, arg1: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<(), dbus::Error>;
    fn ref_(&self) -> Result<(), dbus::Error>;
    fn unref(&self) -> Result<(), dbus::Error>;
    fn clean(&self, arg0: Vec<&str>) -> Result<(), dbus::Error>;
    fn id(&self) -> Result<String, dbus::Error>;
    fn names(&self) -> Result<Vec<String>, dbus::Error>;
    fn following(&self) -> Result<String, dbus::Error>;
    fn requires(&self) -> Result<Vec<String>, dbus::Error>;
    fn requisite(&self) -> Result<Vec<String>, dbus::Error>;
    fn wants(&self) -> Result<Vec<String>, dbus::Error>;
    fn binds_to(&self) -> Result<Vec<String>, dbus::Error>;
    fn part_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn required_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn requisite_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn wanted_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn bound_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn consists_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn conflicts(&self) -> Result<Vec<String>, dbus::Error>;
    fn conflicted_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn before(&self) -> Result<Vec<String>, dbus::Error>;
    fn after(&self) -> Result<Vec<String>, dbus::Error>;
    fn on_failure(&self) -> Result<Vec<String>, dbus::Error>;
    fn triggers(&self) -> Result<Vec<String>, dbus::Error>;
    fn triggered_by(&self) -> Result<Vec<String>, dbus::Error>;
    fn propagates_reload_to(&self) -> Result<Vec<String>, dbus::Error>;
    fn reload_propagated_from(&self) -> Result<Vec<String>, dbus::Error>;
    fn joins_namespace_of(&self) -> Result<Vec<String>, dbus::Error>;
    fn requires_mounts_for(&self) -> Result<Vec<String>, dbus::Error>;
    fn documentation(&self) -> Result<Vec<String>, dbus::Error>;
    fn description(&self) -> Result<String, dbus::Error>;
    fn load_state(&self) -> Result<String, dbus::Error>;
    fn active_state(&self) -> Result<String, dbus::Error>;
    fn sub_state(&self) -> Result<String, dbus::Error>;
    fn fragment_path(&self) -> Result<String, dbus::Error>;
    fn source_path(&self) -> Result<String, dbus::Error>;
    fn drop_in_paths(&self) -> Result<Vec<String>, dbus::Error>;
    fn unit_file_state(&self) -> Result<String, dbus::Error>;
    fn unit_file_preset(&self) -> Result<String, dbus::Error>;
    fn state_change_timestamp(&self) -> Result<u64, dbus::Error>;
    fn state_change_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn inactive_exit_timestamp(&self) -> Result<u64, dbus::Error>;
    fn inactive_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn active_enter_timestamp(&self) -> Result<u64, dbus::Error>;
    fn active_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn active_exit_timestamp(&self) -> Result<u64, dbus::Error>;
    fn active_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn inactive_enter_timestamp(&self) -> Result<u64, dbus::Error>;
    fn inactive_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn can_start(&self) -> Result<bool, dbus::Error>;
    fn can_stop(&self) -> Result<bool, dbus::Error>;
    fn can_reload(&self) -> Result<bool, dbus::Error>;
    fn can_isolate(&self) -> Result<bool, dbus::Error>;
    fn can_clean(&self) -> Result<Vec<String>, dbus::Error>;
    fn job(&self) -> Result<(u32, dbus::Path<'static>), dbus::Error>;
    fn stop_when_unneeded(&self) -> Result<bool, dbus::Error>;
    fn refuse_manual_start(&self) -> Result<bool, dbus::Error>;
    fn refuse_manual_stop(&self) -> Result<bool, dbus::Error>;
    fn allow_isolate(&self) -> Result<bool, dbus::Error>;
    fn default_dependencies(&self) -> Result<bool, dbus::Error>;
    fn on_failure_job_mode(&self) -> Result<String, dbus::Error>;
    fn ignore_on_isolate(&self) -> Result<bool, dbus::Error>;
    fn need_daemon_reload(&self) -> Result<bool, dbus::Error>;
    fn job_timeout_usec(&self) -> Result<u64, dbus::Error>;
    fn job_running_timeout_usec(&self) -> Result<u64, dbus::Error>;
    fn job_timeout_action(&self) -> Result<String, dbus::Error>;
    fn job_timeout_reboot_argument(&self) -> Result<String, dbus::Error>;
    fn condition_result(&self) -> Result<bool, dbus::Error>;
    fn assert_result(&self) -> Result<bool, dbus::Error>;
    fn condition_timestamp(&self) -> Result<u64, dbus::Error>;
    fn condition_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn assert_timestamp(&self) -> Result<u64, dbus::Error>;
    fn assert_timestamp_monotonic(&self) -> Result<u64, dbus::Error>;
    fn conditions(&self) -> Result<Vec<(String, bool, bool, String, i32)>, dbus::Error>;
    fn asserts(&self) -> Result<Vec<(String, bool, bool, String, i32)>, dbus::Error>;
    fn load_error(&self) -> Result<(String, String), dbus::Error>;
    fn transient(&self) -> Result<bool, dbus::Error>;
    fn perpetual(&self) -> Result<bool, dbus::Error>;
    fn start_limit_interval_usec(&self) -> Result<u64, dbus::Error>;
    fn start_limit_burst(&self) -> Result<u32, dbus::Error>;
    fn start_limit_action(&self) -> Result<String, dbus::Error>;
    fn failure_action(&self) -> Result<String, dbus::Error>;
    fn failure_action_exit_status(&self) -> Result<i32, dbus::Error>;
    fn success_action(&self) -> Result<String, dbus::Error>;
    fn success_action_exit_status(&self) -> Result<i32, dbus::Error>;
    fn reboot_argument(&self) -> Result<String, dbus::Error>;
    fn invocation_id(&self) -> Result<Vec<u8>, dbus::Error>;
    fn collect_mode(&self) -> Result<String, dbus::Error>;
    fn refs(&self) -> Result<Vec<String>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Unit for blocking::Proxy<'a, C> {

    fn start(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Start", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn stop(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Stop", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Reload", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Restart", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn try_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "TryRestart", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload_or_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ReloadOrRestart", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn reload_or_try_restart(&self, arg0: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ReloadOrTryRestart", (arg0, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn enqueue_job(&self, arg0: &str, arg1: &str) -> Result<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String, Vec<(u32, dbus::Path<'static>, String, dbus::Path<'static>, String)>), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "EnqueueJob", (arg0, arg1, ))
    }

    fn kill(&self, arg0: &str, arg1: i32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Kill", (arg0, arg1, ))
    }

    fn reset_failed(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "ResetFailed", ())
    }

    fn set_properties(&self, arg0: bool, arg1: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "SetProperties", (arg0, arg1, ))
    }

    fn ref_(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Ref", ())
    }

    fn unref(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Unref", ())
    }

    fn clean(&self, arg0: Vec<&str>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.systemd1.Unit", "Clean", (arg0, ))
    }

    fn id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Id")
    }

    fn names(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Names")
    }

    fn following(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Following")
    }

    fn requires(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Requires")
    }

    fn requisite(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Requisite")
    }

    fn wants(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Wants")
    }

    fn binds_to(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "BindsTo")
    }

    fn part_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "PartOf")
    }

    fn required_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RequiredBy")
    }

    fn requisite_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RequisiteOf")
    }

    fn wanted_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "WantedBy")
    }

    fn bound_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "BoundBy")
    }

    fn consists_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ConsistsOf")
    }

    fn conflicts(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Conflicts")
    }

    fn conflicted_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ConflictedBy")
    }

    fn before(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Before")
    }

    fn after(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "After")
    }

    fn on_failure(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "OnFailure")
    }

    fn triggers(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Triggers")
    }

    fn triggered_by(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "TriggeredBy")
    }

    fn propagates_reload_to(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "PropagatesReloadTo")
    }

    fn reload_propagated_from(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ReloadPropagatedFrom")
    }

    fn joins_namespace_of(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "JoinsNamespaceOf")
    }

    fn requires_mounts_for(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RequiresMountsFor")
    }

    fn documentation(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Documentation")
    }

    fn description(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Description")
    }

    fn load_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "LoadState")
    }

    fn active_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ActiveState")
    }

    fn sub_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "SubState")
    }

    fn fragment_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "FragmentPath")
    }

    fn source_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "SourcePath")
    }

    fn drop_in_paths(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "DropInPaths")
    }

    fn unit_file_state(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "UnitFileState")
    }

    fn unit_file_preset(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "UnitFilePreset")
    }

    fn state_change_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StateChangeTimestamp")
    }

    fn state_change_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StateChangeTimestampMonotonic")
    }

    fn inactive_exit_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "InactiveExitTimestamp")
    }

    fn inactive_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "InactiveExitTimestampMonotonic")
    }

    fn active_enter_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ActiveEnterTimestamp")
    }

    fn active_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ActiveEnterTimestampMonotonic")
    }

    fn active_exit_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ActiveExitTimestamp")
    }

    fn active_exit_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ActiveExitTimestampMonotonic")
    }

    fn inactive_enter_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "InactiveEnterTimestamp")
    }

    fn inactive_enter_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "InactiveEnterTimestampMonotonic")
    }

    fn can_start(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CanStart")
    }

    fn can_stop(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CanStop")
    }

    fn can_reload(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CanReload")
    }

    fn can_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CanIsolate")
    }

    fn can_clean(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CanClean")
    }

    fn job(&self) -> Result<(u32, dbus::Path<'static>), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Job")
    }

    fn stop_when_unneeded(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StopWhenUnneeded")
    }

    fn refuse_manual_start(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RefuseManualStart")
    }

    fn refuse_manual_stop(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RefuseManualStop")
    }

    fn allow_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "AllowIsolate")
    }

    fn default_dependencies(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "DefaultDependencies")
    }

    fn on_failure_job_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "OnFailureJobMode")
    }

    fn ignore_on_isolate(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "IgnoreOnIsolate")
    }

    fn need_daemon_reload(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "NeedDaemonReload")
    }

    fn job_timeout_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "JobTimeoutUSec")
    }

    fn job_running_timeout_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "JobRunningTimeoutUSec")
    }

    fn job_timeout_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "JobTimeoutAction")
    }

    fn job_timeout_reboot_argument(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "JobTimeoutRebootArgument")
    }

    fn condition_result(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ConditionResult")
    }

    fn assert_result(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "AssertResult")
    }

    fn condition_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ConditionTimestamp")
    }

    fn condition_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "ConditionTimestampMonotonic")
    }

    fn assert_timestamp(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "AssertTimestamp")
    }

    fn assert_timestamp_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "AssertTimestampMonotonic")
    }

    fn conditions(&self) -> Result<Vec<(String, bool, bool, String, i32)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Conditions")
    }

    fn asserts(&self) -> Result<Vec<(String, bool, bool, String, i32)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Asserts")
    }

    fn load_error(&self) -> Result<(String, String), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "LoadError")
    }

    fn transient(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Transient")
    }

    fn perpetual(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Perpetual")
    }

    fn start_limit_interval_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StartLimitIntervalUSec")
    }

    fn start_limit_burst(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StartLimitBurst")
    }

    fn start_limit_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "StartLimitAction")
    }

    fn failure_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "FailureAction")
    }

    fn failure_action_exit_status(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "FailureActionExitStatus")
    }

    fn success_action(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "SuccessAction")
    }

    fn success_action_exit_status(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "SuccessActionExitStatus")
    }

    fn reboot_argument(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "RebootArgument")
    }

    fn invocation_id(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "InvocationID")
    }

    fn collect_mode(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "CollectMode")
    }

    fn refs(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Unit", "Refs")
    }
}