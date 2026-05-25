#!/usr/bin/env bash

set -xeuo pipefail

SPEC="coda.openapi.json"
LIB="src/lib.rs"
GEN="src/gen.rs"
SDF="sd --fixed-strings"
TEMP=$(mktemp -d)
SPEC_FORMATTED="$TEMP/coda.openapi.formatted.json"

# download the spec
curl https://coda.io/apis/v1/openapi.json > $SPEC

# fix & format the spec
for ENUM in PageType TableType ControlType; do
    $SDF "$ENUM" "${ENUM}Enum" $SPEC
done
for FIELD in aiCreditsChat aiCreditsBlock aiCreditsColumn aiCreditsAssistant aiCreditsReviewer aiCredits; do
    $SDF "$FIELD," "$FIELD" $SPEC
done
# add the "json" language tag
$SDF '[JSON-LD](https://json-ld.org/) format.\n\n```\n  // Currency' '[JSON-LD](https://json-ld.org/) format.\n\n```json\n  // Currency' $SPEC
jq 'del(.paths["/docs/{docId}/hooks/automation/{ruleId}"].post.requestBody.content["application/x-www-form-urlencoded","text/plain"])' "$SPEC" \
    | jq . > "$SPEC_FORMATTED"
mv "$SPEC_FORMATTED" "$SPEC"

cargo progenitor -i $SPEC -o $TEMP -n coda_api -v 0.1.0
mkdir -p src
rm -f "$GEN"
# #![allow(unreachable_code)] is needed to work around .clone() calls on empty enums
# this line must be at the top of the file
echo "#![allow(unreachable_code)]" >> $GEN
cat $TEMP/$LIB >> $GEN
$SDF '///!' '/// !' $GEN
$SDF '<span>' '' $GEN
$SDF 'go/<name>/<var1>/<var2>' 'go/{name}/{var1}/{var2}' $GEN
$SDF '"http://schema.org/"' '"<http://schema.org/>"' $GEN
$SDF 'https://coda.io/trust/tos' '<https://coda.io/trust/tos>' $GEN
$SDF '\n/// border' ' border' $GEN
$SDF '    Your doc ID' 'Your doc ID' $GEN
$SDF "elided_named_lifetimes" "mismatched_lifetime_syntaxes" $GEN
$SDF "pub struct Client {" "pub struct RawClient {" $GEN
$SDF "impl Client {" "impl RawClient {" $GEN
$SDF "for Client {" "for RawClient {" $GEN
$SDF "for &Client {" "for &RawClient {" $GEN
$SDF "pub use super::Client;" "pub use super::RawClient;" $GEN

# `fix` must be executed before `fix:type`
mise run fix
mise run fix:type "pub enum PushButtonResult" snippets/PushButtonResult "$GEN"
mise run test
