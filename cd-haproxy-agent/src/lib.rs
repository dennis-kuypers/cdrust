use derive_new::new;
use std::fmt::{Formatter, Write};

#[derive(Default)]
pub struct AgentResponse {
    pub state: Option<State>,
    pub max_connections: Option<MaxConnections>,
    pub weight: Option<Weight>,
}

impl std::fmt::Display for AgentResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut any_written = false;

        let mut write = |d: &dyn std::fmt::Display| {
            if any_written {
                f.write_char(',')?;
            }
            d.fmt(f)?;
            any_written = true;

            Ok(())
        };

        if let Some(weight) = &self.weight {
            write(weight)?;
        }
        if let Some(maxconn) = &self.max_connections {
            write(maxconn)?;
        }
        if let Some(state) = &self.state {
            write(state)?;
        }

        f.write_char('\n')?;

        Ok(())
    }
}

pub enum State {
    /// Turns the server into READY mode, thus canceling any DRAIN or MAINT state
    Ready,

    /// Turns the server into DRAIN mode, thus it will not accept any new connections other than those that are accepted via persistence.
    Drain,

    /// Turns the server into MAINT mode, thus it will not accept any new connections at all, and health checks will be stopped.
    Maintenance,

    /// Turns the server into FAIL
    Fail(Option<String>),
    Stopped(Option<String>),

    /// - The word "up" sets back the server's operating state as UP if health checks
    ///   also report that the service is accessible.
    Up,
    /// - The word "down", "fail", or "stopped", optionally followed by a
    ///   description string after a sharp ('#'). All of these mark the server's
    ///   operating state as DOWN, but since the word itself is reported on the stats
    ///   page, the difference allows an administrator to know if the situation was
    ///   expected or not : the service may intentionally be stopped, may appear up
    ///   but fail some validity tests, or may be seen as down (e.g. missing process,
    ///   or port not responding).
    Down(Option<String>),
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[inline] // helper
        fn write_with_reason(f: &mut Formatter<'_>, name: &str, r: &Option<String>) -> std::fmt::Result {
            f.write_str(name)?;
            if let Some(reason) = r {
                f.write_char('#')?;
                f.write_str(reason)?;
            }
            std::fmt::Result::Ok(())
        }

        match self {
            State::Ready => f.write_str("ready"),
            State::Drain => f.write_str("drain"),
            State::Maintenance => f.write_str("maint"),
            State::Down(r) => write_with_reason(f, "down", r),
            State::Fail(r) => write_with_reason(f, "fail", r),
            State::Stopped(r) => write_with_reason(f, "stopped", r),
            State::Up => f.write_str("up"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WeightParseError {
    #[error("Weight must be greater than 0")]
    OutOfRange,
}

#[derive(new)]
pub struct Weight(u32);

impl Weight {
    pub const DEFAULT: Weight = Weight(100);
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)?;
        f.write_char('%')
    }
}

#[derive(new)]
pub struct MaxConnections(u32);

impl std::fmt::Display for MaxConnections {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("maxconn:")?;
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::{AgentResponse, MaxConnections, State};

    #[test]
    fn format_service() {
        let str = AgentResponse {
            max_connections: Some(MaxConnections(100)),
            weight: None,
            state: Some(State::Stopped(Some("abc".to_string()))),
        }
        .to_string();

        assert_eq!("maxconn:100,stopped#abc\n", str)
    }
}
