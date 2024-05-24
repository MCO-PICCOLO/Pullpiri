<!--
SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.

SPDX-License-Identifier: Apache-2.0
-->

<img src="doc/images/Piccolo.jpg" 
width="50%"
height="50%"
/>

# PICCOLO
The main goal of PICCOLO project is to develop an efficient vehicle service orchestrator framework to realize the potential benefits of cloud native technologies for in-vehicle services and applications. In this direction, PICCOLO shall ensure the activation of pre-defined use case scenarios or policies in a well-organized and streamlined fashion depending upon the various contexts of vehicle status, environment, connected devices and service requirements. PICCOLO shall enable the deployment of vehicle scenarios and policies in short development cycle by reducing the development lead time. In addition, it provides necessary management framework for the deployment of micro services as per the requirements of vehicle applications and thus saving the integration costs, time, and efforts.

## Features
- **Package Service on Cloud**  
: Compose an application by combining services  
: Define service relationships, select execution location  
: Application Model Structure (Redundant Structure, Monitoring, Proxy, Etc.)

- **Service Orchestrator on Vehicle**  
: Execute scenarios depending on vehicle status  
: Determination of Service Transition Time (Start, Stop, Restart, Switching, Etc.)  
: Manager text-defined scenarios and policies (Supports UX updates after release)

## Out of Scope
The PICCOLO project does not cover several features, including implementations for basic controlling services like launch, stop, create, or delete, deployment or retrieval of workloads from a cloud registry, management of non-containerized services, and functional areas supported by general container runtime and systemd.

## Based Projects
PICCOLO project has a plan to support several projects as core orchestrator.
Current version of PICCOLO could be running based on this project.

- **[Eclipse Bluechi](https://github.com/eclipse-bluechi/bluechi/tree/main)**

## Getting started
Refer to [Getting Started](/doc/docs/getting-started.md).

## Development
Refer to [Development](/doc/docs/developments.md).

## License
The [LICENSES](/LICENSES) directory contains all the licenses used by the PICCOLO Project.  
Piccolo itself uses the [Apache-2.0](/LICENSES/Apache-2.0.txt) license.

For detail, refer to [license-readme](/LICENSES/README.md).