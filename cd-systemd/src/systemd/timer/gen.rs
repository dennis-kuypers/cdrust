// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1/unit/logrotate_2Etimer -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Timer`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Timer {
    fn unit(&self) -> Result<String, dbus::Error>;
    fn timers_monotonic(&self) -> Result<Vec<(String, u64, u64)>, dbus::Error>;
    fn timers_calendar(&self) -> Result<Vec<(String, String, u64)>, dbus::Error>;
    fn on_clock_change(&self) -> Result<bool, dbus::Error>;
    fn on_timezone_change(&self) -> Result<bool, dbus::Error>;
    fn next_elapse_usec_realtime(&self) -> Result<u64, dbus::Error>;
    fn next_elapse_usec_monotonic(&self) -> Result<u64, dbus::Error>;
    fn last_trigger_usec(&self) -> Result<u64, dbus::Error>;
    fn last_trigger_usec_monotonic(&self) -> Result<u64, dbus::Error>;
    fn result(&self) -> Result<String, dbus::Error>;
    fn accuracy_usec(&self) -> Result<u64, dbus::Error>;
    fn randomized_delay_usec(&self) -> Result<u64, dbus::Error>;
    fn persistent(&self) -> Result<bool, dbus::Error>;
    fn wake_system(&self) -> Result<bool, dbus::Error>;
    fn remain_after_elapse(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Timer for blocking::Proxy<'a, C> {

    fn unit(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "Unit")
    }

    fn timers_monotonic(&self) -> Result<Vec<(String, u64, u64)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "TimersMonotonic")
    }

    fn timers_calendar(&self) -> Result<Vec<(String, String, u64)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "TimersCalendar")
    }

    fn on_clock_change(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "OnClockChange")
    }

    fn on_timezone_change(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "OnTimezoneChange")
    }

    fn next_elapse_usec_realtime(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "NextElapseUSecRealtime")
    }

    fn next_elapse_usec_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "NextElapseUSecMonotonic")
    }

    fn last_trigger_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "LastTriggerUSec")
    }

    fn last_trigger_usec_monotonic(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "LastTriggerUSecMonotonic")
    }

    fn result(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "Result")
    }

    fn accuracy_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "AccuracyUSec")
    }

    fn randomized_delay_usec(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "RandomizedDelayUSec")
    }

    fn persistent(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "Persistent")
    }

    fn wake_system(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "WakeSystem")
    }

    fn remain_after_elapse(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.systemd1.Timer", "RemainAfterElapse")
    }
}
