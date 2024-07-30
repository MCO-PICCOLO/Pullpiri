/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

//use super::workload::podspec::PodSpec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Scenario {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: ScenarioSpec,
    status: Option<ScenarioStatus>,
}

impl Scenario {
    pub fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    pub fn get_conditions(&self) -> Option<Condition> {
        self.spec.conditions.clone()
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.spec.actions.clone()
    }

    pub fn get_targets(&self) -> Vec<Target> {
        self.spec.targets.clone()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct ScenarioSpec {
    conditions: Option<Condition>,
    actions: Vec<Action>,
    targets: Vec<Target>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct ScenarioStatus {
    state: ScenarioState,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
enum ScenarioState {
    None,
    Waiting,
    Running,
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Condition {
    express: String,
    value: String,
    operands: Operand,
}

impl Condition {
    pub fn get_express(&self) -> String {
        self.express.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn get_operand_value(&self) -> String {
        self.operands.value.clone()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Action {
    operation: String,
}

impl Action {
    pub fn get_operation(&self) -> String {
        self.operation.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Operand {
    r#type: String,
    name: String,
    value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Target {
    name: String,
}

impl Target {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
