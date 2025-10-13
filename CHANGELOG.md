<!--
SPDX-FileCopyrightText: 2025 Stackable GmbH
SPDX-License-Identifier: Apache-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Initial release of nifi-migrate tool ([#1], [#2], [#3])
- JoltTransformJSON processor migration ([NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554))
  - Type: `org.apache.nifi.processors.standard.JoltTransformJSON` → `org.apache.nifi.processors.jolt.JoltTransformJSON`
  - Bundle: `nifi-standard-nar` → `nifi-jolt-nar`
  - Property name migrations (5 properties)
- JoltTransformRecord processor migration ([NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554))
  - Type: `org.apache.nifi.processors.jolt.record.JoltTransformRecord` → `org.apache.nifi.processors.jolt.JoltTransformRecord`
  - Bundle: `nifi-jolt-record-nar` → `nifi-jolt-nar`
  - Property name migrations (5 properties)
- Distributed Cache Controller Services migration ([NIFI-13596](https://issues.apache.org/jira/browse/NIFI-13596))
  - `DistributedMapCacheClientService` → `MapCacheClientService`
  - `DistributedSetCacheClientService` → `SetCacheClientService`
  - `DistributedMapCacheServer` → `MapCacheServer`
  - `DistributedSetCacheServer` → `SetCacheServer`

[#1]: https://github.com/stackabletech/nifi-migrate/pull/1
[#2]: https://github.com/stackabletech/nifi-migrate/pull/2
[#3]: https://github.com/stackabletech/nifi-migrate/pull/3
