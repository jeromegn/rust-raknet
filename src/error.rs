#[derive(Debug, thiserror::Error)]
pub enum RaknetError {
    #[error("could not set raw socket")]
    SetRaknetRawSocketError,
    #[error("not listen")]
    NotListen,
    #[error("BindAdreesError")]
    BindAdreesError,
    #[error("ConnectionClosed")]
    ConnectionClosed,
    #[error("NotSupportVersion")]
    NotSupportVersion,
    #[error("IncorrectReply")]
    IncorrectReply,
    #[error("PacketParseError")]
    PacketParseError,
    #[error("SocketError")]
    SocketError,
    #[error("IncorrectReliability")]
    IncorrectReliability,
    #[error("IncorrectPacketID")]
    IncorrectPacketID,
    #[error("ReadPacketBufferError")]
    ReadPacketBufferError,
    #[error("PacketSizeExceedMTU")]
    PacketSizeExceedMTU,
    #[error("PacketHeaderError")]
    PacketHeaderError,
}

pub type Result<T> = std::result::Result<T, RaknetError>;
