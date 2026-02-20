use crate::mpv::{MPV_PROPERTY_PAUSE, MPV_PROPERTY_SPEED, MPV_PROPERTY_TIME_POS, Mpv};
use client::{
    Config,
    client::Client,
    events::{
        ClientEvent::{Info, PropertyChange},
        PlaybackState,
    },
    mediaplayer::MediaPlayer,
};
use std::{
    ffi::{CStr, c_int},
    thread,
    time::Duration,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod mpv;

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(mpv: *mut mpv_handle) -> c_int {
    match inner_main(mpv) {
        Ok(_) => 0,
        _ => -1,
    }
}

#[allow(nonstandard_style)]
fn inner_main(mpv: *mut mpv_handle) -> Result<(), ()> {
    let mpv = Mpv::new(mpv);
    mpv.observe_property(MPV_PROPERTY_PAUSE);
    mpv.observe_property(MPV_PROPERTY_TIME_POS);
    mpv.observe_property(MPV_PROPERTY_SPEED);

    // whether we skip synchronizing towards server because the event follows
    // a sync coming from the server
    // (i didn't yet find a better way to skip MPV events that we caused ourselves)
    let mut skip_seek_sync = true;
    let mut skip_pause_sync = true;
    let mut skip_speed_sync = true;

    let mut dont_fix_desync = false;
    let mut desync_check_ctr = 0;

    // wait a bit before we connect and receive our first state from server
    // until the player is properly initialized (PLAYBACK_RESTART event took place)
    let mut client: Option<Client<Mpv>> = None;
    let mut desired_state: Option<PlaybackState> = None;

    loop {
        let event = mpv.wait_event(0.0);
        let event_id = unsafe { (*event).event_id };

        match event_id {
            mpv_event_id_MPV_EVENT_SHUTDOWN => {
                return Ok(());
            }
            mpv_event_id_MPV_EVENT_SEEK => {
                dont_fix_desync = true;
            }
            mpv_event_id_MPV_EVENT_PROPERTY_CHANGE => unsafe {
                let property = (*event).data as *mut mpv_event_property;
                let name = CStr::from_ptr((*property).name);

                if name == MPV_PROPERTY_PAUSE {
                    if !skip_pause_sync && let Some(ref mut client) = client {
                        _ = client.signal_property_change();
                    }

                    skip_pause_sync = false;
                } else if name == MPV_PROPERTY_SPEED {
                    if !skip_speed_sync && let Some(ref mut client) = client {
                        _ = client.signal_property_change();
                    }

                    skip_speed_sync = false;
                }
            },
            mpv_event_id_MPV_EVENT_NONE => {
                if !dont_fix_desync {
                    if desync_check_ctr % 5000 == 0
                        && let Some(ref desired_state) = desired_state
                    {
                        let desired_pos = desired_state.get_time_pos();
                        if client.is_some()
                            && (desired_pos - mpv.get_time_pos().unwrap_or(desired_pos)).abs() > 0.4
                        {
                            mpv.display("[mpmp] Desync detected, adjusting...");
                            (skip_seek_sync, skip_pause_sync, skip_speed_sync) =
                                apply_desired_state(&desired_state, &mpv);
                        }

                        desync_check_ctr = 0;
                    }

                    desync_check_ctr += 1;
                }
            }
            mpv_event_id_MPV_EVENT_PLAYBACK_RESTART => {
                if client.is_none() {
                    let config = Config::new().map_err(|_| ())?;
                    client = Some(Client::new(&mpv, config))
                }

                if !skip_seek_sync && let Some(ref mut client) = client {
                    _ = client.signal_property_change();
                }

                skip_seek_sync = false;
                dont_fix_desync = false;
            }
            _ => {
                // no event
            }
        }

        if let Some(ref mut client) = client
            && let Ok(events) = client.fetch_events()
        {
            events.into_iter().for_each(|msg| match msg {
                PropertyChange(p) => {
                    (skip_seek_sync, skip_pause_sync, skip_speed_sync) =
                        apply_desired_state(&p, &mpv);
                    desired_state = Some(p);
                }
                Info(ref i) => mpv.display(i),
            });
        }

        thread::sleep(Duration::from_millis(2));
    }
}

fn apply_desired_state(p: &PlaybackState, mpv: &Mpv) -> (bool, bool, bool) {
    let mut skip_seek_sync = false;
    let mut skip_pause_sync = false;
    let mut skip_speed_sync = false;

    if mpv.is_paused().unwrap_or(p.paused) != p.paused {
        skip_pause_sync = true;
        mpv.set_paused(p.paused);
    }

    if mpv.get_speed().unwrap_or(p.speed) != p.speed {
        skip_speed_sync = true;
        mpv.set_speed(p.speed);
    }

    let desired_pos = p.get_time_pos();
    if mpv.get_time_pos().unwrap_or(desired_pos) != desired_pos {
        skip_seek_sync = true;
        mpv.seek(desired_pos);
    }

    (skip_seek_sync, skip_pause_sync, skip_speed_sync)
}
