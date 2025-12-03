// Copyright 2020 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! "Raw" HTTP client APIs.
//!
//! The [`conjure_runtime::Client`] wraps a raw HTTP client, which is used to handle the actual HTTP communication. A
//! default raw client is provided, but this can be overridden if desired.
//!
//! # Behavior
//!
//! The raw client interacts directly with the [`http::Request`] and [`http::Response`] types, with a body type
//! implementing the [`http_body::Body`] trait rather than using the [`AsyncBody`] type. The request's URI is provided in
//! absolute-form, and all headers have already been set in the header map. The HTTP response should be
//! returned directly, without any interpretation of the status code, handling of redirects, etc.
//!
//! A raw client is expected to implement `Service<Request<RawBody>, Response = Response<B>>`, where `B` implements the
//! [`http_body::Body`] trait. The error type returned by the client must implement `Into<Box<dyn Error + Sync + Send>>`
//! and be safe-loggable.
//!
//! Some configuration set in the [`Builder`] affects the raw client, including the connect timeout, security
//! configuration, and proxy configuration. The default raw client respects these settings, but other implementations
//! will need to handle them on their own.
//!
//! [`conjure_runtime::Client`]: crate::Client
//! [`AsyncBody`]: conjure_http::client::AsyncBody
pub use crate::raw::default::*;

mod default;
