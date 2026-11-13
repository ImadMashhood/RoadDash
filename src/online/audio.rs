//! Confirmed host sound events retained briefly and deduplicated on receivers.
use super::*;
use crate::sound::{AudioCue, Cue};

#[derive(Clone, Copy, Serialize, Deserialize)]
enum Runover {
    Body,
    Bike,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Event {
    pub id: u64,
    cue: Cue,
    pub position: [f32; 3],
    // Older clients ignore this field and play the compatible cue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    runover: Option<Runover>,
}

impl Event {
    fn new(id: u64, audio: AudioCue) -> Self {
        let (cue, runover) = match audio.kind {
            Cue::Runover => (Cue::Land, Some(Runover::Body)),
            Cue::RunoverBike => (Cue::Crash, Some(Runover::Bike)),
            cue => (cue, None),
        };
        Self {
            id,
            cue,
            position: audio.position.to_array(),
            runover,
        }
    }

    pub(super) fn audio(&self) -> AudioCue {
        let kind = match self.runover {
            Some(Runover::Body) => Cue::Runover,
            Some(Runover::Bike) => Cue::RunoverBike,
            None => self.cue,
        };
        AudioCue::new(kind, Vec3::from_array(self.position))
    }
}
#[derive(Resource, Default)]
pub(super) struct Events {
    pub pending: VecDeque<Event>,
    serial: u64,
}
pub(super) fn collect(
    mut messages: MessageReader<crate::sound::AudioCue>,
    mut events: ResMut<Events>,
    state: Res<NetState>,
) {
    for cue in messages.read() {
        if matches!(state.connection, Connection::Host(_)) {
            events.serial += 1;
            let id = events.serial;
            events.pending.push_back(Event::new(id, *cue));
        }
    }
    while events.pending.len() > 64 {
        events.pending.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runovers_use_legacy_wire_cues_and_round_trip_their_material() {
        #[derive(Deserialize)]
        struct LegacyEvent {
            cue: Cue,
        }

        let point = Vec3::new(2., 3., 4.);
        for (kind, fallback) in [(Cue::Runover, "Land"), (Cue::RunoverBike, "Crash")] {
            let encoded =
                serde_json::to_string(&Event::new(1, AudioCue::new(kind, point))).unwrap();
            let legacy: LegacyEvent = serde_json::from_str(&encoded).unwrap();
            assert_eq!(
                serde_json::to_string(&legacy.cue).unwrap(),
                format!("\"{fallback}\"")
            );
            let current: Event = serde_json::from_str(&encoded).unwrap();
            assert_eq!(current.audio().position, point);
            assert_eq!(
                std::mem::discriminant(&current.audio().kind),
                std::mem::discriminant(&kind)
            );
        }
        let legacy: Event =
            serde_json::from_str(r#"{"id":1,"cue":"Crash","position":[2,3,4]}"#).unwrap();
        assert!(matches!(legacy.audio().kind, Cue::Crash));
    }
}
