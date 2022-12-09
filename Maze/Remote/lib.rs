//! # Remote Interactions
//!
//! # Library Layout
//!
//! ## Player
//! Contains the definition for a [`player::PlayerProxy`], which enables the
//! [`::referee::referee::Referee`] to communicate with `Client`s over a TCP connection. Also
//! contains a [`players::player::PlayerApi`] implementation for [`player::PlayerProxy`] so it can
//! be used in place of [`players::player::LocalPlayer`] in a [`::referee::player::Player`].
//!
//! ## Referee
//! Contains the definition for a [`refreee::RefereeProxy`], which enables a `Client` to
//! communicate with a remote `Server`.
//!
//! ## Json
//! Contains the data definition for the JSON that is sent between the [`player::PlayerProxy`] and
//! [`refreee::RefereeProxy`]. This module also has methods for conveniently constructing and
//! accessing data inside [`json::JsonMethodCalls`].
//!

/// contains data defintions for remote messages
pub mod json;
/// Contains the PlayerProxy
pub mod player;
/// Contains the RefereeProxy
pub mod referee;
