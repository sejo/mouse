#!/bin/bash
# Get last created tag
LTCOMMIT=$(git rev-list --abbrev-commit --tags --max-count=1)
LTAG=$(git describe --abbrev=0 --tags ${LTCOMMIT} 2>/dev/null || true)
ADC=${DRONE_COMMIT:0:7}

if [[ "${LTCOMMIT}" == "${ADC}" ]]; then
	VERSION="${DRONE_TAG##v}"
else
	VERSION="${LTAG##v}-next-${ADV}"
fi

sed -i "s/0\\.0\\.0/${VERSION}/" Cargo.toml
cat Cargo.toml
