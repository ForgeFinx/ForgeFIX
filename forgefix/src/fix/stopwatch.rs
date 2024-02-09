use crate::fix::session::Event;
use tokio::time::{sleep_until, Duration, Instant, Sleep};

pub(super) struct Timeout {
    next_instant: Instant,
    duration: Duration,
    event: Event,
}

impl Timeout {
    pub(super) fn new(instant: Instant, duration: Duration, event: Event) -> Timeout {
        Timeout {
            next_instant: instant,
            duration,
            event,
        }
    }

    pub(super) fn reset_timeout(&mut self) {
        self.next_instant = Instant::now() + self.duration;
    }

    pub(super) fn set_timeout_duration(&mut self, dur: Duration) {
        self.duration = dur;
        self.reset_timeout();
    }

    pub(super) fn timeout(&self) -> (Sleep, &Event) {
        (sleep_until(self.next_instant), &self.event)
    }
}

pub(super) struct FixTimeouts {
    heartbeat_timeout: Timeout,
    test_request_timeout: Timeout,
    logout_timeout: Timeout,
    awaiting_logout: bool,
}

impl FixTimeouts {
    pub(super) fn new(
        heartbeat_dur: Duration,
        test_request_dur: Duration,
        logout_dur: Duration,
    ) -> FixTimeouts {
        let next_heartbeat_timeout = Instant::now() + heartbeat_dur;
        let next_test_request_timeout = Instant::now() + test_request_dur;
        let next_logout_timeout = Instant::now() + logout_dur;
        let awaiting_logout = false;

        let heartbeat_timeout =
            Timeout::new(next_heartbeat_timeout, heartbeat_dur, Event::SendHeartbeat);
        let test_request_timeout = Timeout::new(
            next_test_request_timeout,
            test_request_dur,
            Event::SendTestRequest(0),
        );
        let logout_timeout = Timeout::new(next_logout_timeout, logout_dur, Event::LogoutExpired);

        FixTimeouts {
            heartbeat_timeout,
            test_request_timeout,
            logout_timeout,
            awaiting_logout,
        }
    }

    pub(super) fn next_expiring_timeout(&mut self) -> &mut Timeout {
        if !self.awaiting_logout
            && self.heartbeat_timeout.next_instant < self.test_request_timeout.next_instant
        {
            &mut self.heartbeat_timeout
        } else if !self.awaiting_logout {
            &mut self.test_request_timeout
        } else {
            &mut self.logout_timeout
        }
    }

    pub(super) fn reset_heartbeat(&mut self) {
        self.heartbeat_timeout.reset_timeout();
    }

    pub(super) fn reset_test_request(&mut self) {
        self.test_request_timeout.reset_timeout();
    }

    pub(super) fn start_logout_timeout(&mut self) {
        self.awaiting_logout = true;
        self.logout_timeout.reset_timeout();
    }

    pub(super) fn set_durations(
        &mut self,
        heartbeat_dur: Duration,
        test_request_dur: Duration,
        logout_dur: Duration,
    ) {
        self.heartbeat_timeout.set_timeout_duration(heartbeat_dur);
        self.heartbeat_timeout.reset_timeout();
        self.test_request_timeout
            .set_timeout_duration(test_request_dur);
        self.test_request_timeout.reset_timeout();
        self.logout_timeout.set_timeout_duration(logout_dur);
        self.logout_timeout.reset_timeout();
    }
}
