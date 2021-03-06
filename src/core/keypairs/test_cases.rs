use crate::constants::ACCOUNT_ID_LENGTH;
use crate::core::addresscodec::utils::SEED_LENGTH;
use crate::core::keypairs::utils::SHA512_HASH_LENGTH;
use crate::core::keypairs::ED25519_SIGNATURE_LENGTH;

pub const TEST_MESSAGE: &str = "test message";

pub const TEST_MESSAGE_SHA: [u8; SHA512_HASH_LENGTH] = [
    149, 11, 42, 126, 255, 167, 143, 81, 166, 53, 21, 236, 69, 224, 62, 206, 190, 80, 239, 47, 28,
    65, 230, 150, 41, 181, 7, 120, 241, 27, 192, 128,
];

pub const TEST_ACCOUNT_ID: [u8; ACCOUNT_ID_LENGTH] = [
    159, 127, 79, 53, 164, 208, 121, 119, 65, 32, 123, 166, 113, 156, 97, 173, 156, 182, 36, 249,
];

pub const SEED_ED25519: &str = "sEdSKaCy2JT7JaM7v95H9SxkhP9wS2r";
pub const SEED_SECP256K1: &str = "sp5fghtJtpUorTwvof1NpDXAzNwf5";

pub const TEST_BYTES: [u8; SEED_LENGTH] =
    *b"\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10";

pub const RAW_PUBLIC_ED25519: &str =
    "EDBB664A14F510A366404BC4352A2230A5608364B3D51105C39D7B652DDEAD3ED3";
pub const RAW_PRIVATE_ED25519: &str =
    "ED08C948F6D4ACCBAD74DD560538F86D400BC610CFA74566E87561AB63FB72DD0F";

pub const PUBLIC_ED25519: &str =
    "ED01FA53FA5A7E77798F882ECE20B1ABC00BB358A9E55A202D0D0676BD0CE37A63";
pub const PRIVATE_ED25519: &str =
    "EDB4C4E046826BD26190D09715FC31F4E6A728204EADD112905B08B14B7F15C4F3";

pub const PUBLIC_SECP256K1: &str =
    "030D58EB48B4420B1F7B9DF55087E0E29FEF0E8468F9A6825B01CA2C361042D435";
pub const PRIVATE_SECP256K1: &str =
    "00D78B9735C3F26501C7337B8A5727FD53A6EFDBC6AA55984F098488561F985E23";

pub const SIGNATURE_ED25519: [u8; ED25519_SIGNATURE_LENGTH] = [
    203, 25, 158, 27, 253, 78, 61, 170, 16, 94, 72, 50, 238, 223, 163, 100, 19, 225, 244, 66, 5,
    228, 239, 185, 226, 126, 130, 96, 68, 194, 30, 62, 46, 132, 139, 188, 129, 149, 232, 149, 155,
    173, 248, 135, 89, 155, 115, 16, 173, 27, 112, 71, 239, 17, 182, 130, 224, 208, 104, 247, 55,
    73, 117, 14,
];

pub const SIGNATURE_SECP256K1: [u8; 70] = [
    48, 68, 2, 32, 88, 58, 145, 201, 94, 84, 230, 166, 81, 196, 123, 236, 34, 116, 78, 11, 16, 30,
    44, 64, 96, 231, 176, 143, 99, 65, 101, 125, 173, 155, 195, 238, 2, 32, 125, 20, 137, 199, 57,
    93, 176, 24, 141, 58, 86, 169, 119, 236, 186, 84, 179, 111, 169, 55, 27, 64, 49, 150, 85, 177,
    180, 66, 158, 51, 239, 45,
];

pub const CLASSIC_ADDRESS_ED25519: &str = "rLUEXYuLiQptky37CqLcm9USQpPiz5rkpD";
pub const CLASSIC_ADDRESS_SECP256K1: &str = "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1";
