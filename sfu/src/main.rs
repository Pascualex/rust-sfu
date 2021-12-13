use std::{sync::Arc, time::Duration};

use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors,
        media_engine::{MediaEngine, MIME_TYPE_VP8},
        APIBuilder,
    },
    error::Result,
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication,
    rtp_transceiver::{
        rtp_codec::{RTCRtpCodecCapability, RTCRtpCodecParameters, RTPCodecType},
        rtp_receiver::RTCRtpReceiver,
    },
    track::{
        track_local::{track_local_static_rtp::TrackLocalStaticRTP, TrackLocal, TrackLocalWriter},
        track_remote::TrackRemote,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut media_engine = MediaEngine::default();

    media_engine.register_codec(
        RTCRtpCodecParameters {
            capability: RTCRtpCodecCapability {
                mime_type: MIME_TYPE_VP8.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: String::new(),
                rtcp_feedback: Vec::new(),
            },
            payload_type: 96,
            ..Default::default()
        },
        RTPCodecType::Video,
    )?;

    let registry = register_default_interceptors(Registry::new(), &mut media_engine).await?;

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let pc = Arc::new(api.new_peer_connection(config).await?);
    let output_track = Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_VP8.to_owned(),
            ..Default::default()
        },
        "track-video".to_owned(),
        "stream-video".to_owned(),
    ));
    let sender = pc
        .add_track(Arc::clone(&output_track) as Arc<dyn TrackLocal + Send + Sync>)
        .await?;
    tokio::spawn(async move {
        let mut rtcp_buf = vec![0u8; 1500];
        while let Ok((_, _)) = sender.read(&mut rtcp_buf).await {}
        println!("video rtp_sender.read loop exit");
        Result::<()>::Ok(())
    });

    let line = must_read_stdin()?;
    let desc_data = decode(line.as_str())?;
    let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data).unwrap();

    pc.set_remote_description(offer).await?;

    let pc_arc = Arc::downgrade(&pc);
    pc.on_track(Box::new(
        move |track: Option<Arc<TrackRemote>>, _: Option<Arc<RTCRtpReceiver>>| {
            let track = match track {
                Some(value) => value,
                None => return Box::pin(async {}),
            };

            if track.kind() != RTPCodecType::Video {
                return Box::pin(async {});
            }

            let media_ssrc = track.ssrc();
            let pc_arc_2 = pc_arc.clone();
            tokio::spawn(async move {
                let mut result = Result::<usize>::Ok(0);
                while result.is_ok() {
                    let timeout = tokio::time::sleep(Duration::from_secs(3));
                    tokio::pin!(timeout);
                    tokio::select! {
                        _ = timeout.as_mut() => {
                            if let Some(pc_arc) = pc_arc_2.upgrade() {
                                result = pc_arc
                                    .write_rtcp(&[Box::new(PictureLossIndication {
                                        sender_ssrc: 0,
                                        media_ssrc,
                                    })])
                                    .await
                                    .map_err(Into::into);
                            } else {
                                break;
                            }
                        }
                    };
                }
            });

            let output_track_2 = Arc::clone(&output_track);
            tokio::spawn(async move {
                println!(
                    "Track has started, of type {}: {}",
                    track.payload_type(),
                    track.codec().await.capability.mime_type
                );
                // Read RTP packets being sent to webrtc-rs
                while let Ok((rtp, _)) = track.read_rtp().await {
                    if let Err(err) = output_track_2.write_rtp(&rtp).await {
                        println!("output track write_rtp got error: {}", err);
                        break;
                    }
                }

                println!(
                    "on_track finished, of type {}: {}",
                    track.payload_type(),
                    track.codec().await.capability.mime_type
                );
            });

            Box::pin(async {})
        },
    ))
    .await;

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(1);

    // Set the handler for Peer connection state
    // This will notify you when the peer has connected/disconnected
    pc.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {}", s);

        if s == RTCPeerConnectionState::Failed {
            // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
            // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
            // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
            println!("Peer Connection has gone to failed exiting");
            let _ = done_tx.try_send(());
        }

        Box::pin(async {})
    }))
    .await;

    // Create an answer
    let answer = pc.create_answer(None).await?;

    // Create channel that is blocked until ICE Gathering is complete
    let mut gather_complete = pc.gathering_complete_promise().await;

    // Sets the LocalDescription, and starts our UDP listeners
    pc.set_local_description(answer).await?;

    // Block until ICE Gathering is complete, disabling trickle ICE
    // we do this because we only can exchange one signaling message
    // in a production application you should exchange ICE Candidates via OnICECandidate
    let _ = gather_complete.recv().await;

    // Output the answer in base64 so we can paste it in browser
    if let Some(local_desc) = pc.local_description().await {
        let json_str = serde_json::to_string(&local_desc).unwrap();
        let b64 = base64::encode(&json_str);
        println!("{}", b64);
    } else {
        println!("generate local_description failed!");
    }

    println!("Press ctrl-c to stop");
    //let timeout = tokio::time::sleep(Duration::from_secs(20));
    //tokio::pin!(timeout);

    tokio::select! {
        //_ = timeout.as_mut() => {
        //    println!("received timeout signal!");
        //}
        _ = done_rx.recv() => {
            println!("received done signal!");
        }
        _ = tokio::signal::ctrl_c() => {
            println!("");
        }
    };

    pc.close().await?;

    Ok(())
}

/// must_read_stdin blocks until input is received from stdin
fn must_read_stdin() -> Result<String> {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_owned();
    println!();

    Ok(line)
}

/// decode decodes the input from base64
/// It can optionally unzip the input after decoding
pub fn decode(s: &str) -> Result<String> {
    let b = base64::decode(s).unwrap();
    let s = String::from_utf8(b).unwrap();
    Ok(s)
}
