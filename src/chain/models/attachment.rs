pub enum Attachment {
    V0, // Will be 0 if transaction version is 0
    V1, // After FluxEnable DigitalGoodsStore
}

pub enum AttachmentType {
    Message,
    EncryptedMessage,
    EncryptToSelfMessage,
    PublicKeyAnnouncement,
}
