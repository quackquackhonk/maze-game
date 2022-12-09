//! # Library Layout
//!
//! ## State
//! Contains the definition for [`state::State`] which is a type that represents the state of a
//! game whether finished or in progress. It is generic over the type of player information stored
//! in it. If it contains [`state::PublicPlayerInfo`] it is a state with public information. If it
//! contains [`state::PrivatePlayerInfo`] it contains information that is not publicly available
//! such as player goals.
//!
//! ## Board
//! A module containing [`board::Board`]
//! ## Grid
//! The [`grid::Grid`] that backs the [`board::Board`] and its helper methods.
//! ## Tile
//! The individual tiles in a [`grid::Grid`] inside a [`board::Board`].
//!
//! Also contains the definitions for [`tile::ConnectorShape`] and [`tile::CompassDirection`],
//! the latter of which is reused in other modules to describe directions.
//!
//! ## Gem
//! The enum containing all the possible gems and their image representations.
//!
//! ## Color
//! A module containing the [`color::Color`] data type
//!
//!
//! ### Json
//! [`json`] contains a lot of shared Json definitions. Many of these are used in integration tests
//! but a few keys ones, are part of the definitions for the Json sent over TCP to enable the
//! Remote interactions of `RefereeProxy`s and `PlayerProxy`s

/// Contains all the types needed for the Board State and mutating the `Board`
pub mod board;
/// Containts the types needed to represent colors.
pub mod color;
/// Contains the enum including all the possible Gems
pub mod gem;
/// Contains types for the `Grid` type and its `Position` type for indexing
pub mod grid;
/// Contains all the utilities for serializing and deserializing from JSON
pub mod json;
/// Contains all the types needed for State
pub mod state;
/// Contains the Tile type for use in the `Board`
pub mod tile;
