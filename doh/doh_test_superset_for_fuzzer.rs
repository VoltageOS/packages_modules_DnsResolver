/*
 * Copyright (C) 2022, The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! It is only currently intended for fuzzing testing.

/// The Rust FFI bindings to C APIs for implementation of doh.
pub mod boot_time;
mod config;
mod connection;
mod dispatcher;
mod encoding;
mod ffi;
mod network;
/// The Rust FFI bindings to C APIs for implementation of doh frontend.
mod tests;
