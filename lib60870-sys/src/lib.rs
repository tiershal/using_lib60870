#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::CS101_CauseOfTransmission;
pub use ffi::IEC60870_QOI_STATION;
pub use ffi::IEC_60870_5_104_DEFAULT_PORT;

pub struct CommonAddr(pub i32);

pub struct Timestamp {
    value: ffi::sCP56Time2a,
}

impl Timestamp {
    pub fn now_ms() -> Self {
        let mut timestamp = Timestamp::default();

        unsafe {
            let ms = ffi::Hal_getTimeInMs();
            ffi::CP56Time2a_createFromMsTimestamp(&mut timestamp.value, ms as u64);
        }

        timestamp
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            value: ffi::sCP56Time2a {
                encodedValue: [0; 7],
            },
        }
    }
}

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        let mut timestamp = Timestamp::default();

        unsafe {
            ffi::CP56Time2a_createFromMsTimestamp(&mut timestamp.value, value);
        }

        timestamp
    }
}

pub enum QualifierOfInterrogation {
    Station,
    Group1,
    Group2,
    Group3,
    Group4,
    Group5,
    Group6,
    Group7,
    Group8,
    Group9,
    Group10,
    Group11,
    Group12,
    Group13,
    Group14,
    Group15,
    Group16,
}

impl QualifierOfInterrogation {
    fn as_u8(&self) -> u8 {
        let val = match self {
            QualifierOfInterrogation::Station => ffi::IEC60870_QOI_STATION,
            QualifierOfInterrogation::Group1 => ffi::IEC60870_QOI_GROUP_1,
            QualifierOfInterrogation::Group2 => ffi::IEC60870_QOI_GROUP_2,
            QualifierOfInterrogation::Group3 => ffi::IEC60870_QOI_GROUP_3,
            QualifierOfInterrogation::Group4 => ffi::IEC60870_QOI_GROUP_4,
            QualifierOfInterrogation::Group5 => ffi::IEC60870_QOI_GROUP_5,
            QualifierOfInterrogation::Group6 => ffi::IEC60870_QOI_GROUP_6,
            QualifierOfInterrogation::Group7 => ffi::IEC60870_QOI_GROUP_7,
            QualifierOfInterrogation::Group8 => ffi::IEC60870_QOI_GROUP_8,
            QualifierOfInterrogation::Group9 => ffi::IEC60870_QOI_GROUP_9,
            QualifierOfInterrogation::Group10 => ffi::IEC60870_QOI_GROUP_10,
            QualifierOfInterrogation::Group11 => ffi::IEC60870_QOI_GROUP_11,
            QualifierOfInterrogation::Group12 => ffi::IEC60870_QOI_GROUP_12,
            QualifierOfInterrogation::Group13 => ffi::IEC60870_QOI_GROUP_13,
            QualifierOfInterrogation::Group14 => ffi::IEC60870_QOI_GROUP_14,
            QualifierOfInterrogation::Group15 => ffi::IEC60870_QOI_GROUP_15,
            QualifierOfInterrogation::Group16 => ffi::IEC60870_QOI_GROUP_16,
        };

        val as u8
    }
}

pub mod hal {
    use crate::ffi;

    pub fn get_time_in_ms() -> u64 {
        unsafe { ffi::Hal_getTimeInMs() as u64 }
    }
}

pub mod cs104 {
    use std::net::IpAddr;

    use crate::{ffi, QualifierOfInterrogation};

    #[derive(Debug)]
    pub struct ConnectionBuilder {
        ip: IpAddr,
        port: Option<u16>,
        local_ip: Option<IpAddr>,
        local_port: Option<u16>,
    }

    impl ConnectionBuilder {
        pub fn new(ip: IpAddr) -> Self {
            Self {
                ip,
                port: None,
                local_ip: None,
                local_port: None,
            }
        }

        pub fn with_port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }

        pub fn with_local_ip(mut self, local_ip: IpAddr) -> Self {
            self.local_ip = Some(local_ip);
            self
        }

        pub fn with_local_port(mut self, local_port: u16) -> Self {
            self.local_port = Some(local_port);
            self
        }

        pub fn build(self) -> Connection<Disconnected> {
            let connection = unsafe {
                ffi::CS104_Connection_create(
                    self.ip.to_string().as_ptr() as *const i8,
                    self.port
                        .unwrap_or(ffi::IEC_60870_5_104_DEFAULT_PORT as u16)
                        as i32,
                )
            };

            unsafe {
                if let Some(local_ip) = self.local_ip {
                    ffi::CS104_Connection_setLocalAddress(
                        connection,
                        local_ip.to_string().as_ptr() as *const i8,
                        self.local_port.unwrap_or(0) as i32,
                    );
                }
            }

            Connection {
                connection,
                _state: std::marker::PhantomData,
            }
        }
    }

    #[derive(Debug, ::thiserror::Error)]
    pub enum ConnectionError {
        #[error("{0}")]
        Failed(String),
    }

    pub struct Connected;
    pub struct Disconnected;

    pub struct Connection<T> {
        pub(crate) connection: ffi::CS104_Connection,
        _state: std::marker::PhantomData<T>,
    }

    impl<Disconnected> Connection<Disconnected> {
        #[must_use]
        pub fn connect(&self) -> Result<Connection<Connected>, ConnectionError> {
            unsafe {
                let connected = ffi::CS104_Connection_connect(self.connection);

                if !connected {
                    return Err(ConnectionError::Failed("Failed to connect".to_string()));
                }
            }

            Ok(Connection {
                connection: self.connection,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<Connected> Connection<Connected> {
        #[must_use]
        pub fn disconnect(self) -> Connection<Disconnected> {
            unsafe {
                ffi::CS104_Connection_close(self.connection);
            }

            Connection {
                connection: self.connection,
                _state: std::marker::PhantomData,
            }
        }

        pub fn start_transmission(&self) {
            unsafe {
                ffi::CS104_Connection_sendStartDT(self.connection);
            }
        }

        pub fn stop_transmission(&self) {
            unsafe {
                ffi::CS104_Connection_sendStopDT(self.connection);
            }
        }

        pub fn send_interrogation_command(
            &self,
            command: ffi::CS101_CauseOfTransmission,
            qualifier: QualifierOfInterrogation,
            server_addr: crate::CommonAddr,
        ) {
            unsafe {
                ffi::CS104_Connection_sendInterrogationCommand(
                    self.connection,
                    command,
                    server_addr.0,
                    qualifier.as_u8(),
                );
            }
        }

        pub fn send_test_command_with_timestamp(
            &self,
            command: u16,
            timestamp: crate::Timestamp,
            server_addr: crate::CommonAddr,
        ) {
            unsafe {
                // SAFETY: The CS104_Connection_sendTestCommandWithTimestamp function takes in a *mut sCP56Time2a but
                // if you follow the calls in the C code, you'll see that it doesn't modify the value. Instead, it just
                // dereferences the pointer to copy the value. This is safe because the value is not modified.
                let mut timestamp_value = timestamp.value;

                ffi::CS104_Connection_sendTestCommandWithTimestamp(
                    self.connection,
                    server_addr.0,
                    command,
                    &mut timestamp_value,
                );
            }
        }
    }

    impl<T> Drop for Connection<T> {
        fn drop(&mut self) {
            unsafe {
                // SAFETY: The connection is owned by this struct and is not used after this call.
                // The connection is also closed as part of the call to CS104_Connection_destroy.
                ffi::CS104_Connection_destroy(self.connection);
            }
        }
    }
}
