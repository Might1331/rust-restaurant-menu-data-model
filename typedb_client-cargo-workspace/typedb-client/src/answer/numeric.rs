/*
 * Copyright (C) 2022 Vaticle
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

#[derive(Clone, Debug)]
pub enum Numeric {
    Long(i64),
    Double(f64),
    NaN,
}

impl Numeric {
    pub fn into_i64(self) -> i64 {
        if let Self::Long(value) = self {
            value
        } else {
            panic!()
        }
    }

    pub fn into_f64(self) -> f64 {
        if let Self::Double(value) = self {
            value
        } else {
            panic!()
        }
    }
}

impl From<Numeric> for i64 {
    fn from(n: Numeric) -> Self {
        n.into_i64()
    }
}

impl From<Numeric> for f64 {
    fn from(n: Numeric) -> Self {
        n.into_f64()
    }
}
