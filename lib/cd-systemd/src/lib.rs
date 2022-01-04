/*!
Communicate via dbus with systemd.

```
let conn = SystemdConnection::new_system().unwrap();
conn.do_stuff().unwrap();
```

# Features
The `dbus_wrapper` (default: false) feature re-exports the dbus wrappers as `dbus_wrapper` to expose
more functionality. You can then use the [dbus](https://crates.io/crates/dbus) crate with those wrappers.

# References
* [systemd dbus documentation](https://www.man7.org/linux/man-pages/man5/org.freedesktop.systemd1.5.html)

*/
mod systemd;

pub use systemd::models::*;

#[cfg(feature = "dbus_wrapper")]
pub mod dbus_wrapper {
    pub use crate::systemd::manager;
    pub use crate::systemd::service;
    pub use crate::systemd::timer;
    pub use crate::systemd::unit;
}

pub type Result<T> = std::result::Result<T, SystemdError>;

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(5000);
const DBUS_DESTINATION: &str = "org.freedesktop.systemd1";
const _PATH_FMT_STR: &str = "/org/freedesktop/systemd1/{}/{}";

pub struct SystemdConnection {
    dbus: dbus::blocking::SyncConnection,
}

impl SystemdConnection {
    pub fn new_system() -> Result<Self> {
        let dbus = dbus::blocking::SyncConnection::new_system().map_err(|e| SystemdError::DbusFailure(e))?;
        Ok(SystemdConnection { dbus })
    }

    pub fn service_get_state(&self, service: &UnitName) -> Result<ActiveState> {
        use systemd::unit::Unit;

        let proxy = self.dbus.with_proxy(
            DBUS_DESTINATION,
            format!("/org/freedesktop/systemd1/unit/{}_2Eservice", service),
            DEFAULT_TIMEOUT,
        );

        let state_string = proxy.active_state().map_err(|e| SystemdError::DbusFailure(e))?;
        state_string.parse().map_err(|_| SystemdError::UnexpectedResponse)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SystemdError {
    #[error(transparent)]
    DbusFailure(#[from] dbus::Error),
    #[error("Systemd response not understood")]
    UnexpectedResponse,
}
