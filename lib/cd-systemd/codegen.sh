# This codegen is missing .path .device .automount .slice .scope .job .socket

# The generated code needs a `use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;` statement.
# for flexibility we use the `gen_prelude`
#echo 'use crate::gen_prelude::*;' > src/systemd/manager/gen.rs
dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1 -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Manager >> src/systemd/manager/gen.rs
#echo 'use crate::gen_prelude::*;' > src/systemd/unit/gen.rs
dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1/unit/ssh_2Eservice -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Unit >> src/systemd/unit/gen.rs
#echo 'use crate::gen_prelude::*;' > src/systemd/service/gen.rs
dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1/unit/ssh_2Eservice -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Service >> src/systemd/service/gen.rs
#echo 'use crate::gen_prelude::*;' > src/systemd/timer/gen.rs
dbus-codegen-rust -s -d org.freedesktop.systemd1 -p /org/freedesktop/systemd1/unit/logrotate_2Etimer -i org.freedesktop.systemd1 -f org.freedesktop.systemd1.Timer >> src/systemd/timer/gen.rs
