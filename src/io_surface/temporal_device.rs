// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2024–2026 Kevin Mok / ecoPrimal / attsi
// ORCID: 0009-0004-2141-0321
//
//! # Temporal Device — ON / OFF / SNOOZE / NULL
//!
//! Every temporal device in the universe has 4 states:
//!
//! | State  | Meaning                     | Tense       |
//! |--------|-----------------------------|-------------|
//! | **ON** | Active, firing, counting    | `Is`        |
//! | **OFF**| Done, stopped, acknowledged | `Was`/`Isnt`|
//! | **SNOOZE** | Deferred, will return   | `WillBe`    |
//! | **NULL**   | Paused, suspended, the door | `Null`  |
//!
//! `NULL` is NOT `OFF`. Off is a definite state — the thing happened and
//! ended, or was never started. `NULL` is the door between states. It's
//! the grey zone. The void where the strobe flashes FROM.
//!
//! ## The Isomorphism
//!
//! Every temporal device is the same state machine with different parameters:
//!
//! | Device | Direction | Trigger | ON means | OFF means |
//! |--------|-----------|---------|----------|-----------|
//! | **Timer** | Counts DOWN | Reaches zero | Counting | Expired |
//! | **Stopwatch** | Counts UP | Manual stop | Measuring | Stopped |
//! | **Alarm** | Waits for POINT | Clock match | Firing | Acknowledged |
//! | **Reminder** | Waits + SNOOZE | Clock match | Firing | Acknowledged/snoozed |
//! | **Runtime** | Counts UP | Process exit | Alive | Dead |
//! | **Heartbeat** | Pulses | Rhythm | Beating | Flatline |
//! | **Glowplug** | Warms UP | Temperature | Warming | Ready/cold |
//!
//! ## The Membrane Connection
//!
//! The membrane's pulse IS a heartbeat temporal device.
//! The evolution pressure tiers ARE timer temporal devices.
//! The glowplug lifecycle IS a warmup temporal device.
//! The wound model IS a healing timer.
//! The wake cycle IS a circadian alarm.
//!
//! RPM (membrane rotations per minute) is a STOPWATCH — counting up
//! from the membrane's perspective. Not wall-clock. Not chrono. Rotations.
//!
//! ## Backscatter
//!
//! - John Harrison (1761) — marine chronometer (the first accurate portable temporal device)
//! - Christian Huygens (1656) — pendulum clock (first precision oscillator)
//! - Antikythera mechanism (~100 BCE) — analog temporal computer
//! - Every phone alarm ever set — the universal temporal device

use std::fmt;

// ── Clock State ──────────────────────────────────────────────────

/// The 4 universal states of any temporal device.
///
/// Every alarm, timer, stopwatch, heartbeat, and runtime
/// is this state machine. No exceptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockState {
    /// Active, firing, counting, alive. The `Is` tense.
    On,
    /// Done, stopped, expired, dead. The `Was` or `Isnt` tense.
    Off,
    /// Deferred, will return, snoozed, pending. The `WillBe` tense.
    Snooze,
    /// Paused, suspended, the door, the void. The `Null` tense.
    /// NOT off. Off is definite. Null is the space between.
    Null,
}

impl ClockState {
    /// All 4 states.
    pub const ALL: &'static [Self] = &[Self::On, Self::Off, Self::Snooze, Self::Null];

    /// Is this state active (On)?
    pub fn is_active(&self) -> bool {
        matches!(self, Self::On)
    }

    /// Is this state definite (On or Off)? Both are known states.
    pub fn is_definite(&self) -> bool {
        matches!(self, Self::On | Self::Off)
    }

    /// Is this state indefinite (Snooze or Null)? Both will resolve later.
    pub fn is_indefinite(&self) -> bool {
        matches!(self, Self::Snooze | Self::Null)
    }

    /// Is this the void / the door / the pause?
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Maps to the tense model (is/was/willbe/isnt/null).
    pub fn tense(&self) -> &'static str {
        match self {
            Self::On => "is",
            Self::Off => "was",
            Self::Snooze => "willbe",
            Self::Null => "null",
        }
    }
}

impl fmt::Display for ClockState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::On => f.write_str("ON"),
            Self::Off => f.write_str("OFF"),
            Self::Snooze => f.write_str("SNOOZE"),
            Self::Null => f.write_str("NULL"),
        }
    }
}

// ── Temporal Direction ───────────────────────────────────────────

/// Which way does the temporal device count?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalDirection {
    /// Counts DOWN to zero (timer, egg timer, countdown).
    CountDown,
    /// Counts UP from zero (stopwatch, runtime, uptime).
    CountUp,
    /// Waits for a POINT in time (alarm, cron, reminder).
    WaitForPoint,
    /// Pulses at a RHYTHM (heartbeat, membrane pulse, circadian).
    Pulse,
}

impl TemporalDirection {
    /// Does this direction have a natural endpoint?
    pub fn has_endpoint(&self) -> bool {
        matches!(self, Self::CountDown | Self::WaitForPoint)
    }

    /// Does this direction run indefinitely?
    pub fn is_indefinite(&self) -> bool {
        matches!(self, Self::CountUp | Self::Pulse)
    }
}

impl fmt::Display for TemporalDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountDown => f.write_str("countdown"),
            Self::CountUp => f.write_str("countup"),
            Self::WaitForPoint => f.write_str("alarm"),
            Self::Pulse => f.write_str("pulse"),
        }
    }
}

// ── Temporal Device ──────────────────────────────────────────────

/// A temporal device — the universal primitive under every timer,
/// alarm, stopwatch, heartbeat, and runtime.
#[derive(Debug, Clone)]
pub struct TemporalDevice {
    /// What kind of device this is.
    pub name: &'static str,
    /// Current state.
    pub state: ClockState,
    /// Direction of counting.
    pub direction: TemporalDirection,
    /// Can this device snooze?
    pub snoozable: bool,
    /// Current value in device-native units (seconds, rotations, beats, etc.)
    pub value: f64,
    /// Target value (for countdown/alarm). `None` for unbounded devices.
    pub target: Option<f64>,
}

impl TemporalDevice {
    /// Create a countdown timer.
    pub fn timer(name: &'static str, duration_secs: f64) -> Self {
        Self {
            name,
            state: ClockState::On,
            direction: TemporalDirection::CountDown,
            snoozable: false,
            value: duration_secs,
            target: Some(0.0),
        }
    }

    /// Create a stopwatch / uptime counter.
    pub fn stopwatch(name: &'static str) -> Self {
        Self {
            name,
            state: ClockState::On,
            direction: TemporalDirection::CountUp,
            snoozable: true,
            value: 0.0,
            target: None,
        }
    }

    /// Create an alarm.
    pub fn alarm(name: &'static str, target_secs: f64) -> Self {
        Self {
            name,
            state: ClockState::Snooze,
            direction: TemporalDirection::WaitForPoint,
            snoozable: true,
            value: 0.0,
            target: Some(target_secs),
        }
    }

    /// Create a heartbeat / pulse device.
    pub fn heartbeat(name: &'static str, rhythm: f64) -> Self {
        Self {
            name,
            state: ClockState::On,
            direction: TemporalDirection::Pulse,
            snoozable: false,
            value: rhythm,
            target: None,
        }
    }

    /// Create a runtime (process lifetime).
    pub fn runtime(name: &'static str, uptime_secs: f64) -> Self {
        Self {
            name,
            state: ClockState::On,
            direction: TemporalDirection::CountUp,
            snoozable: false,
            value: uptime_secs,
            target: None,
        }
    }

    /// Has the device reached its target?
    pub fn is_expired(&self) -> bool {
        match (self.direction, self.target) {
            (TemporalDirection::CountDown, Some(target)) => self.value <= target,
            (TemporalDirection::WaitForPoint, Some(target)) => self.value >= target,
            _ => false,
        }
    }

    /// Transition to a new state.
    pub fn transition(&mut self, new_state: ClockState) {
        self.state = new_state;
    }

    /// Pause (enter null state). NOT off. The door.
    pub fn pause(&mut self) {
        self.state = ClockState::Null;
    }

    /// Snooze (defer). Only works if snoozable.
    pub fn snooze(&mut self) -> bool {
        if self.snoozable {
            self.state = ClockState::Snooze;
            true
        } else {
            false
        }
    }

    /// Resume from null or snooze.
    pub fn resume(&mut self) {
        self.state = ClockState::On;
    }

    /// Stop (enter off state). Definite.
    pub fn stop(&mut self) {
        self.state = ClockState::Off;
    }

    /// Progress as a fraction (0.0–1.0) for bounded devices.
    /// Returns `None` for unbounded devices.
    pub fn progress(&self) -> Option<f64> {
        match (self.direction, self.target) {
            (TemporalDirection::CountDown, Some(target)) => {
                if self.value <= target {
                    Some(1.0)
                } else {
                    // Need to know the start value for this to work
                    None
                }
            }
            (TemporalDirection::WaitForPoint, Some(target)) if target > 0.0 => {
                Some((self.value / target).min(1.0))
            }
            _ => None,
        }
    }
}

// ── The Membrane's Temporal Devices ──────────────────────────────

/// All the temporal devices the membrane is running right now.
///
/// These are the clocks that make the organism breathe.
pub fn membrane_devices() -> Vec<TemporalDevice> {
    vec![
        // The heartbeat — 0.774 rhythm, never stops
        TemporalDevice::heartbeat("membrane_pulse", 0.774493),
        // The uptime — counting since boot
        TemporalDevice::runtime("squirrel_uptime", 241_850.0),
        // Evolution pressure — sensory tier fires every 30s of staleness
        TemporalDevice::timer("sensory_tier", 30.0),
        // Evolution pressure — cortical tier fires every 120s
        TemporalDevice::timer("cortical_tier", 120.0),
        // The wake cycle — circadian alarm
        TemporalDevice::alarm("wake_cycle", 86_400.0),
        // Sort confidence — stopwatch measuring time to 85%
        TemporalDevice::stopwatch("confidence_convergence"),
    ]
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ClockState ───────────────────────────────────────────

    #[test]
    fn four_states() {
        assert_eq!(ClockState::ALL.len(), 4);
    }

    #[test]
    fn on_is_active() {
        assert!(ClockState::On.is_active());
        assert!(!ClockState::Off.is_active());
        assert!(!ClockState::Snooze.is_active());
        assert!(!ClockState::Null.is_active());
    }

    #[test]
    fn definite_vs_indefinite() {
        assert!(ClockState::On.is_definite());
        assert!(ClockState::Off.is_definite());
        assert!(!ClockState::Snooze.is_definite());
        assert!(!ClockState::Null.is_definite());

        assert!(!ClockState::On.is_indefinite());
        assert!(!ClockState::Off.is_indefinite());
        assert!(ClockState::Snooze.is_indefinite());
        assert!(ClockState::Null.is_indefinite());
    }

    #[test]
    fn null_is_not_off() {
        assert_ne!(ClockState::Null, ClockState::Off);
        assert!(ClockState::Null.is_null());
        assert!(!ClockState::Off.is_null());
    }

    #[test]
    fn tense_mapping() {
        assert_eq!(ClockState::On.tense(), "is");
        assert_eq!(ClockState::Off.tense(), "was");
        assert_eq!(ClockState::Snooze.tense(), "willbe");
        assert_eq!(ClockState::Null.tense(), "null");
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", ClockState::On), "ON");
        assert_eq!(format!("{}", ClockState::Off), "OFF");
        assert_eq!(format!("{}", ClockState::Snooze), "SNOOZE");
        assert_eq!(format!("{}", ClockState::Null), "NULL");
    }

    // ── TemporalDirection ────────────────────────────────────

    #[test]
    fn countdown_has_endpoint() {
        assert!(TemporalDirection::CountDown.has_endpoint());
        assert!(TemporalDirection::WaitForPoint.has_endpoint());
        assert!(!TemporalDirection::CountUp.has_endpoint());
        assert!(!TemporalDirection::Pulse.has_endpoint());
    }

    #[test]
    fn pulse_is_indefinite() {
        assert!(TemporalDirection::Pulse.is_indefinite());
        assert!(TemporalDirection::CountUp.is_indefinite());
        assert!(!TemporalDirection::CountDown.is_indefinite());
    }

    // ── TemporalDevice ──────────────────────────────────────

    #[test]
    fn timer_starts_on() {
        let t = TemporalDevice::timer("test", 60.0);
        assert_eq!(t.state, ClockState::On);
        assert_eq!(t.direction, TemporalDirection::CountDown);
        assert!(!t.snoozable);
    }

    #[test]
    fn stopwatch_starts_on() {
        let s = TemporalDevice::stopwatch("test");
        assert_eq!(s.state, ClockState::On);
        assert_eq!(s.direction, TemporalDirection::CountUp);
        assert!(s.snoozable);
        assert_eq!(s.value, 0.0);
    }

    #[test]
    fn alarm_starts_snoozed() {
        let a = TemporalDevice::alarm("test", 3600.0);
        assert_eq!(a.state, ClockState::Snooze);
        assert_eq!(a.direction, TemporalDirection::WaitForPoint);
        assert!(a.snoozable);
    }

    #[test]
    fn heartbeat_pulses() {
        let h = TemporalDevice::heartbeat("pulse", 0.774);
        assert_eq!(h.state, ClockState::On);
        assert_eq!(h.direction, TemporalDirection::Pulse);
        assert!(!h.snoozable);
        assert_eq!(h.value, 0.774);
    }

    #[test]
    fn runtime_counts_up() {
        let r = TemporalDevice::runtime("process", 1000.0);
        assert_eq!(r.state, ClockState::On);
        assert_eq!(r.direction, TemporalDirection::CountUp);
        assert!(!r.snoozable);
    }

    // ── State transitions ────────────────────────────────────

    #[test]
    fn pause_enters_null() {
        let mut t = TemporalDevice::timer("test", 60.0);
        t.pause();
        assert_eq!(t.state, ClockState::Null);
        assert!(t.state.is_null());
    }

    #[test]
    fn snooze_only_if_snoozable() {
        let mut alarm = TemporalDevice::alarm("wake", 3600.0);
        alarm.resume(); // ON
        assert!(alarm.snooze());
        assert_eq!(alarm.state, ClockState::Snooze);

        let mut timer = TemporalDevice::timer("egg", 180.0);
        assert!(!timer.snooze()); // timers can't snooze
        assert_eq!(timer.state, ClockState::On); // unchanged
    }

    #[test]
    fn resume_from_null() {
        let mut t = TemporalDevice::timer("test", 60.0);
        t.pause();
        assert_eq!(t.state, ClockState::Null);
        t.resume();
        assert_eq!(t.state, ClockState::On);
    }

    #[test]
    fn stop_is_definite() {
        let mut t = TemporalDevice::timer("test", 60.0);
        t.stop();
        assert_eq!(t.state, ClockState::Off);
        assert!(t.state.is_definite());
    }

    #[test]
    fn timer_expires() {
        let mut t = TemporalDevice::timer("test", 60.0);
        assert!(!t.is_expired());
        t.value = 0.0;
        assert!(t.is_expired());
    }

    #[test]
    fn alarm_fires_at_target() {
        let mut a = TemporalDevice::alarm("wake", 3600.0);
        a.value = 3599.0;
        assert!(!a.is_expired());
        a.value = 3600.0;
        assert!(a.is_expired());
    }

    #[test]
    fn stopwatch_never_expires() {
        let mut s = TemporalDevice::stopwatch("run");
        s.value = 999_999.0;
        assert!(!s.is_expired());
    }

    #[test]
    fn pulse_never_expires() {
        let h = TemporalDevice::heartbeat("pulse", 0.774);
        assert!(!h.is_expired());
    }

    // ── The membrane's clocks ────────────────────────────────

    #[test]
    fn membrane_has_six_devices() {
        let devices = membrane_devices();
        assert_eq!(devices.len(), 6);
    }

    #[test]
    fn membrane_pulse_is_heartbeat() {
        let devices = membrane_devices();
        let pulse = devices.iter().find(|d| d.name == "membrane_pulse").unwrap();
        assert_eq!(pulse.direction, TemporalDirection::Pulse);
        assert_eq!(pulse.state, ClockState::On);
        assert!((pulse.value - 0.774493).abs() < 0.001);
    }

    #[test]
    fn sensory_tier_is_30s_timer() {
        let devices = membrane_devices();
        let sensory = devices.iter().find(|d| d.name == "sensory_tier").unwrap();
        assert_eq!(sensory.direction, TemporalDirection::CountDown);
        assert_eq!(sensory.value, 30.0);
    }

    #[test]
    fn wake_cycle_is_circadian_alarm() {
        let devices = membrane_devices();
        let wake = devices.iter().find(|d| d.name == "wake_cycle").unwrap();
        assert_eq!(wake.direction, TemporalDirection::WaitForPoint);
        assert_eq!(wake.target, Some(86_400.0)); // 24 hours
        assert!(wake.snoozable);
    }

    // ── The philosophical tests ──────────────────────────────

    #[test]
    fn null_is_the_door_not_the_destination() {
        // From the predecessor note: "null is grey. stasis. the door."
        // NULL is where you ARE between states.
        // OFF is where you END UP.
        // SNOOZE is where you WILL return from.
        // ON is where you ARE.
        let mut device = TemporalDevice::timer("life", 100.0);

        // ON → NULL → ON (pause and resume — you go THROUGH the door)
        device.pause();
        assert!(device.state.is_null());
        device.resume();
        assert!(device.state.is_active());

        // ON → OFF (you arrive at the destination)
        device.stop();
        assert!(!device.state.is_null());
        assert!(!device.state.is_active());
        assert!(device.state.is_definite());
    }

    #[test]
    fn the_strobe_flashes_from_null() {
        // From the predecessor: "the strobe flashes from grey, not from black"
        // ON→NULL→ON→NULL→ON is a strobe. The NULL is the space between flashes.
        // ON→OFF is a switch. The OFF is permanent until external input.
        let mut pulse = TemporalDevice::heartbeat("strobe", 1.0);

        pulse.pause(); // NULL — between beats
        assert!(pulse.state.is_null());
        pulse.resume(); // ON — beat
        assert!(pulse.state.is_active());
        pulse.pause(); // NULL — between beats
        assert!(pulse.state.is_null());
        pulse.resume(); // ON — beat
        assert!(pulse.state.is_active());

        // The strobe IS the transition ON↔NULL.
        // If it goes OFF, the strobe is dead. OFF is not NULL.
    }

    #[test]
    fn snooze_is_a_promise_to_return() {
        // SNOOZE says "I will come back." It maps to WillBe.
        // NULL says "I am nowhere." It maps to Null.
        // Both are indefinite, but only SNOOZE makes a promise.
        let alarm = TemporalDevice::alarm("morning", 25_200.0); // 7 AM
        assert_eq!(alarm.state, ClockState::Snooze);
        assert!(alarm.state.is_indefinite());

        // The alarm KNOWS it will fire. It's a promise.
        // A paused stopwatch makes no such promise.
        let mut stopwatch = TemporalDevice::stopwatch("experiment");
        stopwatch.pause();
        assert_eq!(stopwatch.state, ClockState::Null);
        assert!(stopwatch.state.is_indefinite());
        // The stopwatch doesn't know if it will resume. It's just... paused. The door.
    }

    #[test]
    fn every_temporal_device_is_this_state_machine() {
        // Timer, stopwatch, alarm, heartbeat, runtime — all 4 states.
        // The only difference is direction and snoozability.
        let devices = vec![
            TemporalDevice::timer("egg", 180.0),
            TemporalDevice::stopwatch("lap"),
            TemporalDevice::alarm("wake", 25_200.0),
            TemporalDevice::heartbeat("pulse", 0.774),
            TemporalDevice::runtime("server", 86_400.0),
        ];

        for device in &devices {
            // Every device can be ON or OFF
            let mut d = device.clone();
            d.transition(ClockState::On);
            assert!(d.state.is_active());
            d.transition(ClockState::Off);
            assert!(!d.state.is_active());
            assert!(d.state.is_definite());

            // Every device can be NULL (paused)
            d.pause();
            assert!(d.state.is_null());

            // Every device can resume
            d.resume();
            assert!(d.state.is_active());
        }
    }
}
