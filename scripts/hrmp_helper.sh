# Helper script to open HRMP channels between Trappist, Stout and Asset Hub.
# This script is meant to be run after the relay chain and parachains are spawned.

# Change for your bin path
POLKADOT_BIN=~/projects/trappist/bin/polkadot
POLKADOT_PARACHAIN_BIN=~/projects/trappist/bin/polkadot-parachain

function ensure_binaries() {
    echo "*** Checking for required binaries"
    if [[ ! -f "$POLKADOT_BIN" ]]; then
        echo "  Required polkadot binary '$POLKADOT_BIN' does not exist!"
        echo "  You need to build it and copy to this location!"
        exit 1
    fi
    if [[ ! -f "$POLKADOT_PARACHAIN_BIN" ]]; then
        echo "  Required polkadot-parachain binary '$POLKADOT_PARACHAIN_BIN' does not exist!"
        echo "  You need to build it and copy to this location!"
        exit 1
    fi
    echo "*** All required binaries are present"
}

function ensure_polkadot_js_api() {
    echo "*** Checking for required polkadot-js-api"
    if ! which polkadot-js-api &>/dev/null; then
        echo ''
        echo 'Required command `polkadot-js-api` not in PATH, please, install, e.g.:'
        echo "npm install -g @polkadot/api-cli"
        echo "      or"
        echo "yarn global add @polkadot/api-cli"
        echo ''
        exit 1
    fi
}

function open_hrmp_channels() {
    local relay_url=$1
    local relay_chain_seed=$2
    local sender_para_id=$3
    local recipient_para_id=$4
    local max_capacity=$5
    local max_message_size=$6
    echo "  calling open_hrmp_channels:"
    echo "      relay_url: ${relay_url}"
    echo "      relay_chain_seed: ${relay_chain_seed}"
    echo "      sender_para_id: ${sender_para_id}"
    echo "      recipient_para_id: ${recipient_para_id}"
    echo "      max_capacity: ${max_capacity}"
    echo "      max_message_size: ${max_message_size}"
    echo "      params:"
    echo "--------------------------------------------------"
    polkadot-js-api \
        --ws "${relay_url?}" \
        --seed "${relay_chain_seed?}" \
        --sudo \
        tx.hrmp.forceOpenHrmpChannel \
        ${sender_para_id} \
        ${recipient_para_id} \
        ${max_capacity} \
        ${max_message_size}
}

# Check for binaries
ensure_binaries

# Check for polkadot-js-api cli
ensure_polkadot_js_api

# # HRMP: Trappist - Stout
# open_hrmp_channels \
#     "ws://127.0.0.1:9900" \
#     "//Alice" \
#     1836 3000 4 524288

# # HRMP: Stout - Trappist
# open_hrmp_channels \
#     "ws://127.0.0.1:9900" \
#     "//Alice" \
#     3000 1836 4 524288

# HRMP: Trappist - Asset Hub
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1836 1000 4 524288

# HRMP: Asset Hub - Trappist
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1000 1836 4 524288

# # HRMP: Stout - Asset Hub
# open_hrmp_channels \
#     "ws://127.0.0.1:9900" \
#     "//Alice" \
#     3000 1000 4 524288

# # HRMP: Asset Hub - Stout
# open_hrmp_channels \
#     "ws://127.0.0.1:9900" \
#     "//Alice" \
#     1000 3000 4 524288

# HRMP: Coretime - Asset Hub
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1005 1000 4 524288

# HRMP: Asset Hub - Coretime
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1000 1005 4 524288

# HRMP: Coretime - Trappist
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1005 1836 4 524288

# HRMP: Trappist - Coretime
open_hrmp_channels \
    "ws://127.0.0.1:9900" \
    "//Alice" \
    1836 1005 4 524288
