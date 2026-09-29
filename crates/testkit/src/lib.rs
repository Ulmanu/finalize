//! Shared test support for Momentum.
//!
//! Fixtures and in-memory adapters live here so that unit, integration, and (later) browser
//! tests use the same canonical setup helpers.

#![forbid(unsafe_code)]

pub mod areas;
pub mod auth;
pub mod calendar;
pub mod commands;
pub mod fixtures;
pub mod logbook;
pub mod materializer;
pub mod projects;
pub mod recurring;
pub mod reminders;
pub mod tags;
pub mod todos;
