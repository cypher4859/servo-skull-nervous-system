use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// raw MSP framing (for logs / debugging)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MspPacketV2 {
    pub cmd: u16,           // MSP function ID
    pub flags: u8,          // MSPv2 flags
    pub payload: Vec<u8>,   // raw bytes
    pub crc: u8,            // payload CRC8
    pub dir: MspDir,        // ->FC or <-FC
    pub ts_ms: u64,         // host timestamp
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MspDir { ToFc, FromFc }

// Normalized snapshot (single “truth” that get's handed to other parts of the system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub ts_ms: u64,                 // host timestamp (ms since UNIX epoch)
    pub status: FcStatus,
    pub rc: Option<RcChannels>,     // if you subscribe RC
    pub attitude: Option<Attitude>, // deg
    pub rates: Option<Rates>,       // gyro dps / accel g
    pub battery: Option<Battery>,
    pub altitude: Option<Altitude>, // baro/GPS if available
    pub gps: Option<Gps>,           // if GPS hardware present
    pub link: Option<Link>,         // RSSI/LQ if available
    pub cpu_load: Option<u8>,       // % (Betaflight STATUS_EX)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcStatus {
    pub armed: bool,
    pub failsafe: bool,
    pub flight_mode: FlightModeFlags,
    pub arming_flags: ArmingFlags,
    pub loop_time_us: Option<u32>,
}

// TODO: These bitflags need to be matched up exactly to what the FC is going to return
// They're probably correct but needs confirmed/verified
bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct FlightModeFlags: u32 {
        const ANGLE     = 1 << 0;
        const HORIZON   = 1 << 1;
        const MAG       = 1 << 2;
        const BARO      = 1 << 3;
        const GPS_HOLD  = 1 << 4;
        const GPS_HOME  = 1 << 5;
        const ACRO      = 1 << 6;
    }
}


// TODO: These bitflags need to be matched up exactly to what the FC is going to return
// They're probably correct but needs confirmed/verified
bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ArmingFlags: u32 {
        const OK_TO_ARM     = 1 << 0;
        const PREVENT_ARM   = 1 << 1;
        const MSP_ACTIVE    = 1 << 2;
        const CLI_ACTIVE    = 1 << 3;
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcChannels {
    pub roll: u16,     // 1000-2000us
    pub pitch: u16,
    pub yaw: u16,
    pub throttle: u16,
    pub aux: Vec<u16>, // AUX1..n
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attitude {
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32, // 0..360 or -180..180 - need to pick one and document it
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rates {
    pub gyro_dps: Vec3,   // deg/s
    pub accel_g: Vec3,    // g
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    pub voltage_v: f32,
    pub current_a: Option<f32>,
    pub mah_drawn: Option<u32>,
    pub cells: Option<u8>,
    pub low: bool,
    pub critical: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Altitude {
    pub baro_cm: Option<i32>,
    pub v_speed_cms: Option<i32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gps {
    pub fix: bool,
    pub sats: u8,
    pub lat_e7: i32,   // 1e-7 degrees
    pub lon_e7: i32,
    pub alt_cm: i32,
    pub ground_speed_cms: u32,
    pub ground_course_deg: u16,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub rssi: Option<u16>,     // 0..1023 or vendor-specific
    pub lq: Option<u8>,        // link quality percent
    pub protocol: Option<String>, // e.g., "CRSF", "SBUS" (I think this is SBUS for serial)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    Armed { ts_ms: u64 },
    Disarmed { ts_ms: u64 },
    ModeChanged { ts_ms: u64, flight_mode: FlightModeFlags },
    BatteryLow { ts_ms: u64, voltage_v: f32 },
    Failsafe { ts_ms: u64, active: bool },
}
