#!/bin/bash

rm -f challenge* response* new_challenge* processed*

POWER=16
BATCH=10000
CURVE="bls12_377"

powersoftau="cargo run --bin powersoftau -- --curve-kind $CURVE --batch-size $BATCH --power $POWER"
phase2="cargo run --release --bin prepare_phase2 -- --curve-kind $CURVE --batch-size $BATCH --power $POWER --phase2-size $POWER"
snark="cargo run --release --bin aleo-snark-setup --"

####### Phase 1

$powersoftau new --challenge-fname challenge
yes | $powersoftau contribute --challenge-fname challenge --response-fname response
rm challenge # no longer needed

###### Prepare Phase 2

$phase2 --response-fname response --phase2-fname processed --phase2-size $POWER

###### Phase 2

$snark new --phase1 processed --output initial_ceremony --phase1-size $POWER --is-inner

cp initial_ceremony contribution1
yes | $snark contribute --data contribution1
$snark verify --before initial_ceremony --after contribution1

# a new contributor contributes
cp contribution1 contribution2
yes | $snark contribute --data contribution2
$snark verify --before contribution1 --after contribution2
$snark verify --before initial_ceremony --after contribution2

# done! since `verify` passed, you can be sure that this will work
# as shown in the `mpc.rs` example
