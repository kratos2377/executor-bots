#![allow(unused_imports)]
#![doc = r""]
#![doc = r" Auto-generated IDL types, manual edits do not persist (see `crates/drift-idl-gen`)"]
#![doc = r""]
use self::traits::ToAccountMetas;
use anchor_lang::{
    prelude::{
        account,
        borsh::{self},
        error_code, event, msg, AnchorDeserialize, AnchorSerialize, InitSpace,
    },
    Discriminator,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
pub mod traits {
    use solana_sdk::instruction::AccountMeta;
    #[doc = r" This is distinct from the anchor_lang version of the trait"]
    #[doc = r" reimplemented to ensure the types used are from `solana`` crates _not_ the anchor_lang vendored versions which may be lagging behind"]
    pub trait ToAccountMetas {
        fn to_account_metas(&self) -> Vec<AccountMeta>;
    }
}
pub mod instructions {
    #![doc = r" IDL instruction types"]
    use super::{types::*, *};
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Initialize {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Initialize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeGame {
        pub game_id: [u8; 16],
        pub session_id: [u8; 21],
        pub total_money_staked: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeGame {
        const DISCRIMINATOR: [u8; 8] = [44, 62, 102, 247, 126, 208, 130, 215];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeGame {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateGameStatus {
        pub game_id: [u8; 16],
        pub session_id: [u8; 21],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateGameStatus {
        const DISCRIMINATOR: [u8; 8] = [31, 175, 127, 242, 51, 244, 172, 185];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateGameStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateGameIsSettledStatus {
        pub game_id: [u8; 16],
        pub session_id: [u8; 21],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateGameIsSettledStatus {
        const DISCRIMINATOR: [u8; 8] = [130, 213, 29, 94, 199, 244, 24, 193];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateGameIsSettledStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePlayerBet {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub session_id: [u8; 21],
        pub total_money_staked: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePlayerBet {
        const DISCRIMINATOR: [u8; 8] = [197, 0, 43, 236, 111, 153, 159, 239];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePlayerBet {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UserBet {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub session_id: [u8; 21],
        pub money_staked: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UserBet {
        const DISCRIMINATOR: [u8; 8] = [250, 141, 121, 127, 113, 52, 188, 61];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UserBet {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserBet {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub session_id: [u8; 21],
        pub money_staked: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserBet {
        const DISCRIMINATOR: [u8; 8] = [173, 183, 108, 235, 185, 54, 199, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserBet {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleAllBetsForInvalidGame {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub session_id: [u8; 21],
        pub is_player: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleAllBetsForInvalidGame {
        const DISCRIMINATOR: [u8; 8] = [139, 255, 201, 226, 182, 198, 253, 15];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleAllBetsForInvalidGame {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleAllBets {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub session_id: [u8; 21],
        pub winner_id: [u8; 16],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleAllBets {
        const DISCRIMINATOR: [u8; 8] = [4, 146, 216, 187, 88, 134, 160, 253];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleAllBets {}
}
pub mod types {
    #![doc = r" IDL types"]
    use super::*;
    use std::ops::Mul;
    #[doc = ""]
    #[doc = " backwards compatible u128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned"]
    #[doc = " https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8"]
    #[derive(
        Default,
        PartialEq,
        AnchorSerialize,
        AnchorDeserialize,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        bytemuck :: Zeroable,
        bytemuck :: Pod,
        Debug,
    )]
    #[repr(C)]
    pub struct u128(pub [u8; 16]);
    impl u128 {
        #[doc = " convert self into the std `u128` type"]
        pub fn as_u128(&self) -> std::primitive::u128 {
            std::primitive::u128::from_le_bytes(self.0)
        }
    }
    impl From<std::primitive::u128> for self::u128 {
        fn from(value: std::primitive::u128) -> Self {
            Self(value.to_le_bytes())
        }
    }
    #[doc = " backwards compatible i128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned"]
    #[doc = " https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8"]
    #[derive(
        Default,
        PartialEq,
        AnchorSerialize,
        AnchorDeserialize,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        bytemuck :: Zeroable,
        bytemuck :: Pod,
        Debug,
    )]
    #[repr(C)]
    pub struct i128(pub [u8; 16]);
    impl i128 {
        #[doc = " convert self into the std `i128` type"]
        pub fn as_i128(&self) -> core::primitive::i128 {
            core::primitive::i128::from_le_bytes(self.0)
        }
    }
    impl From<core::primitive::i128> for i128 {
        fn from(value: core::primitive::i128) -> Self {
            Self(value.to_le_bytes())
        }
    }
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq, Debug)]
    pub struct Signature(pub [u8; 64]);
    impl Default for Signature {
        fn default() -> Self {
            Self([0_u8; 64])
        }
    }
    impl serde::Serialize for Signature {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> std::result::Result<S::Ok, S::Error> {
            serializer.serialize_bytes(&self.0)
        }
    }
    impl<'de> serde::Deserialize<'de> for Signature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
            let s = <&[u8]>::deserialize(d)?;
            s.try_into()
                .map(Signature)
                .map_err(serde::de::Error::custom)
        }
    }
    impl anchor_lang::Space for Signature {
        const INIT_SPACE: usize = 8 * 64;
    }
    #[doc = " wrapper around fixed array types used for padding with `Default` implementation"]
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq)]
    pub struct Padding<const N: usize>([u8; N]);
    impl<const N: usize> Default for Padding<N> {
        fn default() -> Self {
            Self([0u8; N])
        }
    }
    impl<const N: usize> std::fmt::Debug for Padding<N> {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    impl<const N: usize> anchor_lang::Space for Padding<N> {
        const INIT_SPACE: usize = 8 * N;
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum BetType {
        #[default]
        WIN,
        LOSE,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum GameSettleType {
        #[default]
        HostDisconnected,
        GameOver,
    }
}
pub mod accounts {
    #![doc = r" IDL Account types"]
    use super::{types::*, *};
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct VortexState {
        pub admin: Pubkey,
        pub signer: Pubkey,
        pub signer_nonce: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for VortexState {
        const DISCRIMINATOR: [u8; 8] = [124, 78, 33, 107, 103, 163, 129, 76];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for VortexState {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for VortexState {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for VortexState {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for VortexState {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for VortexState {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct Game {
        pub game_id: [u8; 16],
        pub pubkey: Pubkey,
        pub total_pot: u64,
        pub is_game_active: bool,
        pub game_vault_key: Pubkey,
        pub is_settled: bool,
        pub session_id: [u8; 21],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Game {
        const DISCRIMINATOR: [u8; 8] = [27, 90, 166, 125, 74, 100, 121, 18];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Game {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Game {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Game {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Game {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Game {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct UserGameBet {
        pub game_id: [u8; 16],
        pub user_bet_wallet_key: Pubkey,
        pub user_betting_on_id: [u8; 16],
        pub bet_type: BetType,
        pub money_staked: u64,
        pub is_settled: bool,
        pub session_id: [u8; 21],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UserGameBet {
        const DISCRIMINATOR: [u8; 8] = [139, 132, 147, 81, 217, 8, 128, 248];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UserGameBet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UserGameBet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UserGameBet {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UserGameBet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UserGameBet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PlayerTotalBet {
        pub game_id: [u8; 16],
        pub user_betting_on_id: [u8; 16],
        pub player_staked_money: u64,
        pub total_money_staked_on_player: u64,
        pub session_id: [u8; 21],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlayerTotalBet {
        const DISCRIMINATOR: [u8; 8] = [200, 243, 110, 109, 109, 162, 242, 214];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlayerTotalBet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlayerTotalBet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlayerTotalBet {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlayerTotalBet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlayerTotalBet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct Initialize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub quote_asset_mint: Pubkey,
        pub vortex_signer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [131, 246, 167, 36, 232, 249, 207, 142];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Initialize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Initialize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Initialize {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for Initialize {}
    #[automatically_derived]
    impl ToAccountMetas for Initialize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.quote_asset_mint,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.vortex_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Initialize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Initialize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeGame {
        pub game: Pubkey,
        pub game_mint: Pubkey,
        pub game_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub admin: Pubkey,
        pub vortex_signer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeGame {
        const DISCRIMINATOR: [u8; 8] = [157, 15, 33, 80, 156, 138, 36, 142];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeGame {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeGame {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeGame {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeGame {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeGame {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_mint,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.vortex_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeGame {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeGame {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateGameStatus {
        pub game: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateGameStatus {
        const DISCRIMINATOR: [u8; 8] = [39, 174, 45, 79, 142, 76, 167, 108];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateGameStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateGameStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateGameStatus {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateGameStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateGameStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateGameStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateGameStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateGameIsSettledStatus {
        pub game: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateGameIsSettledStatus {
        const DISCRIMINATOR: [u8; 8] = [191, 99, 60, 42, 196, 253, 127, 96];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateGameIsSettledStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateGameIsSettledStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateGameIsSettledStatus {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateGameIsSettledStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateGameIsSettledStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateGameIsSettledStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateGameIsSettledStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializePlayerBet {
        pub player_total_bet: Pubkey,
        pub user_bet: Pubkey,
        pub game: Pubkey,
        pub game_mint: Pubkey,
        pub game_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub admin: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePlayerBet {
        const DISCRIMINATOR: [u8; 8] = [71, 67, 39, 186, 106, 231, 161, 229];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePlayerBet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePlayerBet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePlayerBet {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePlayerBet {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePlayerBet {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.player_total_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_mint,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePlayerBet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePlayerBet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UserBet {
        pub user_bet: Pubkey,
        pub player_total_bet: Pubkey,
        pub game: Pubkey,
        pub game_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub user_bet_wallet_key: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UserBet {
        const DISCRIMINATOR: [u8; 8] = [180, 131, 8, 241, 60, 243, 46, 63];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UserBet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UserBet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UserBet {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UserBet {}
    #[automatically_derived]
    impl ToAccountMetas for UserBet {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.player_total_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_bet_wallet_key,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UserBet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UserBet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserBet {
        pub user_bet: Pubkey,
        pub game: Pubkey,
        pub player_total_bet: Pubkey,
        pub game_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub user_bet_wallet_key: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserBet {
        const DISCRIMINATOR: [u8; 8] = [224, 64, 59, 138, 171, 200, 69, 93];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserBet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserBet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserBet {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserBet {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserBet {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.player_total_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_bet_wallet_key,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserBet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserBet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleAllBetsForInvalidGame {
        pub user_bet: Pubkey,
        pub game: Pubkey,
        pub player_bet: Pubkey,
        pub game_vault: Pubkey,
        pub vortex_state: Pubkey,
        pub to: Pubkey,
        pub vortex_signer: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleAllBetsForInvalidGame {
        const DISCRIMINATOR: [u8; 8] = [79, 227, 228, 125, 68, 122, 7, 117];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleAllBetsForInvalidGame {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleAllBetsForInvalidGame {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleAllBetsForInvalidGame {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleAllBetsForInvalidGame {}
    #[automatically_derived]
    impl ToAccountMetas for SettleAllBetsForInvalidGame {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.player_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.vortex_state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.to,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.vortex_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleAllBetsForInvalidGame {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleAllBetsForInvalidGame {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleAllBets {
        pub user_bet: Pubkey,
        pub game: Pubkey,
        pub player_bet: Pubkey,
        pub game_vault: Pubkey,
        pub vortex_state: Pubkey,
        pub to: Pubkey,
        pub vortex_signer: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleAllBets {
        const DISCRIMINATOR: [u8; 8] = [161, 105, 205, 118, 77, 200, 217, 206];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleAllBets {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleAllBets {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleAllBets {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleAllBets {}
    #[automatically_derived]
    impl ToAccountMetas for SettleAllBets {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.player_bet,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.game_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.vortex_state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.to,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.vortex_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleAllBets {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleAllBets {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
}
pub mod errors {
    #![doc = r" IDL error types"]
    use super::{types::*, *};
    #[derive(PartialEq)]
    #[error_code]
    pub enum ErrorCode {
        #[msg("Src Balance < LP Deposit Amount.")]
        NotEnoughBalance,
        #[msg("Pool Mint Amount < 0 on LP Deposit")]
        NoPoolMintOutput,
        #[msg("Trying to burn too much")]
        BurnTooMuch,
        #[msg("Not enough out")]
        NotEnoughOut,
        #[msg("Already Made a Bet on the game")]
        AlreadyMadeABetOnGame,
        #[msg("Bet Type cannot be changed")]
        UserHasDifferentBetType,
        #[msg("You lost the bet. No amount will be rewarded")]
        UserLostTheBet,
        #[msg("Failed Unwrap")]
        FailedUnwrap,
        #[msg("Unable to load AccountLoader")]
        UnableToLoadAccountLoader,
        #[msg("DefaultError")]
        DefaultError,
        #[msg("InvalidPDASigner")]
        InvalidPDASigner,
        #[msg("InvalidPDA")]
        InvalidPDA,
        #[msg("Error During Math Computation")]
        MathError,
        #[msg("Game is still going on")]
        GameIsStillGoingOn,
        #[msg("Game has ended")]
        GameHasEnded,
        #[msg("Only Admin can settle bets")]
        OnlyAdminCanSettleBets,
        #[msg("Only Admin can change game states")]
        OnlyAdminCanChangeGameStates,
        #[msg("Only Admin can initialize")]
        OnlyAdminCanInitialize,
        #[msg("Non zero transfer fee")]
        NonZeroTransferFee,
    }
}
pub mod events {
    #![doc = r" IDL event types"]
    use super::{types::*, *};
}
