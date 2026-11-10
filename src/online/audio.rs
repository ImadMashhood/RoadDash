//! Confirmed host sound events retained briefly and deduplicated on receivers.
use super::*;
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Event {
    pub id: u64,
    pub cue: crate::sound::Cue,
    pub position: [f32; 3],
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
            events.pending.push_back(Event {
                id,
                cue: cue.kind,
                position: cue.position.to_array(),
            });
        }
    }
    while events.pending.len() > 64 {
        events.pending.pop_front();
    }
}
